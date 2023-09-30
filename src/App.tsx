import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
import './styles/App.css';
import './styles/media.css';

function App() {

  interface NextResponse {
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
    };

    const saveItemListener = async () => {
      console.log('save-file event emitted');
      const response: NextResponse = await invoke('save', {index: mediaIndex});
    };

    const previousItemListener = () => {
    };

    const nextItemListener = async () => {
      console.log('started nextitemListner');
      console.log('mediaIndex before: '+mediaIndex);
      let temp = mediaIndex+1;
      const next: NextResponse = await invoke('next', { path: 'str', index: temp });
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

    const firstItemListener = () => {
      console.log('first-item event emitted');
    };

    const lastItemListener = () => {
      console.log('last-item event emitted');
    };

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
