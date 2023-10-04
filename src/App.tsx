import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/dialog';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
import './styles/App.css';
import './styles/media.css';


function App() {

  interface backendResponse {
    index: number;
    media: string;
    name: string;
  }
  
  const [imgSrc, setImgSrc] = useState("");                         // Store the image source URL
  const [errorMessage, setErrorMessage] = useState("");             // Store error messages
  let   [mediaIndex, setMediaIndex] = useState(0);                  // Initialize the value to 0

  /* MEDIA MANAGEMENT */
  const readFileContents = async () => {
    try {
      const selectedPath = await open({ multiple: false, title: 'Open Text File' });
      if (!selectedPath) {
        return null;
      } else {
        return selectedPath as string;
      }
    } catch (error) {
      console.error(error);
      return null;
    }
  };

  function displayMedia(base64Data: string) {
    try {
      const binaryData = atob(base64Data);
      const arrayBuffer = new ArrayBuffer(binaryData.length);
      const uint8Array = new Uint8Array(arrayBuffer);
      for (let i = 0; i < binaryData.length; i++) {
        uint8Array[i] = binaryData.charCodeAt(i);
      }
      const blob = new Blob([uint8Array], { type: 'image/gif' });
      const url = URL.createObjectURL(blob);
      setImgSrc(url);
    } catch (error) {
      console.error(error);
      setErrorMessage("Failed to display the GIF.");
    }
  }

  const getMedia = async () => {
    const path = await readFileContents();
    if (path) {
      try {
        await invoke('sync', {path: path});
        const base64Data = await getBase64(path);
        displayMedia(base64Data);
      } catch (error) {
        console.error(error);
        setErrorMessage("Failed to decode the Media.");
      }
    } else {
      setErrorMessage("No file selected.");
    }
  };

  async function getBase64(path: string) {
    const base64Data: string = await invoke('get_base64', { path });
    return base64Data;
  }

  /* LISTENERS */
  useEffect(() => {
    console.log('test console log twice?');
    const newContentListener = () => {
      console.log('new-content event emitted');
    };

    const openFileListener = async () => {
      console.log('open-file event emitted');
      getMedia();
    };

    const saveItemListener = async () => {
      console.log('save-file event emitted');
      if (!imgSrc) {
        setErrorMessage("No current media to proceed with the action.");
        return;
    }
      const response: backendResponse = await invoke('save', {index: mediaIndex});
    };

    const previousItemListener = async () => {
      console.log('previous event emitted');
      if (!imgSrc) {
        setErrorMessage("No current media to proceed with the action.");
        return;
    }
      let temp;
      if (mediaIndex <= 0) {
        temp = 0;
      } else {
        temp = mediaIndex-1;
      }
      const previous: backendResponse = await invoke('previous', { path: 'str', index: temp });
      let newIndex: number = previous.index;
      const media: string = previous.media;
      try {
        if (media) {
          displayMedia(media);
        } else {
          setErrorMessage("Received invalid media data.");
        }
      } catch (error) {
        console.error(error);
        setErrorMessage("Failed to update the media.");
      }
      setMediaIndex(newIndex);
      console.log('mediaIndex after: '+mediaIndex);
    };

    const nextItemListener = async () => {
      console.log('started nextitemListner');
      console.log('mediaIndex before: '+mediaIndex);
      if (!imgSrc) {
        setErrorMessage("No current media to proceed with the action.");
        return;
      }
      let temp = mediaIndex+1;
      const next: backendResponse = await invoke('next', { path: 'str', index: temp });
      let newIndex: number = next.index;
      const media: string = next.media;
      try {
        if (media) {
          displayMedia(media);
        } else {
          setErrorMessage("Received invalid media data.");
        }
      } catch (error) {
        console.error(error);
        setErrorMessage("Failed to update the media.");
      }
      setMediaIndex(newIndex);
      console.log('mediaIndex after: '+mediaIndex);
    };

    const firstItemListener = async () => {
      console.log('first-item event emitted');
      if (!imgSrc) {
        setErrorMessage("No current media to proceed with the action.");
        return;
      }
    };

    const lastItemListener = async () => {
      if (!imgSrc) {
        setErrorMessage("No current media to proceed with the action.");
        return;
      }
      console.log('last-item event emitted');
    };

    const fileDropListener = async (file: fileDropEvent) => {
      console.log(file);
      const path: string[] = file.payload;
      console.log('path: '+path);
      if (path.length == 1) {
          try {
            console.log('in handlefiledrop');
            await invoke('sync', {path: path[0]});
            const base64Data = await getBase64(path[0]);
            displayMedia(base64Data);
          } catch (error) {
            console.error(error);
            setErrorMessage("Failed to decode the Media.");
          }
        } else {
          setErrorMessage("Not yet implemented.");
        }
    };
  
    const printListener = async () => {
      console.log('print event emitted');
    };
    
    appWindow.listen('tauri://file-drop', fileDropListener);
    appWindow.listen('new-content',   newContentListener);
    appWindow.listen('open-file',     openFileListener);
    appWindow.listen('save-file',     saveItemListener);
    appWindow.listen('previous-item', previousItemListener);
    appWindow.listen('next-item',     nextItemListener);
    appWindow.listen('first-item',    firstItemListener);
    appWindow.listen('last-item',     lastItemListener);
  }, [mediaIndex]);

  /* RETURN */
  return (
    <div className="container">
      <div>
        <button type="button" onClick={() => getMedia()}> READ FILE CONTENTS </button>
      </div>
      {imgSrc && (
        <div>
          <p>Decoded Media:</p>
          <img src={imgSrc} alt="Decoded Media" className="decoded-media" />
        </div>
      )}
      {errorMessage && (
        <div>
          <p>Error:</p>
          <p>{errorMessage}</p>
        </div>
      )}
      <div>counter: {mediaIndex}</div>
    </div>
  );
}

export default App;
