import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';

import './styles/App.css';

function App() {

  const [imgSrc, setImgSrc] = useState("");                 // Store the image source URL
  const [errorMessage, setErrorMessage] = useState("");     // Store error messages

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

  function displayGif(base64Data: string) {
    try {
      const binaryData = atob(base64Data);
      const arrayBuffer = new ArrayBuffer(binaryData.length);
      const uint8Array = new Uint8Array(arrayBuffer);
      for (let i = 0; i < binaryData.length; i++) {
        uint8Array[i] = binaryData.charCodeAt(i);
      }

      const blob = new Blob([uint8Array], { type: 'image/gif' });
      const url = URL.createObjectURL(blob);

      // Set the image source
      setImgSrc(url);
    } catch (error) {
      console.error(error);
      setErrorMessage("Failed to display the GIF.");
    }
  }

  const getGif = async () => {
    const path = await readFileContents();
    if (path) {
      try {
        const base64Data = await getBase64(path);
        displayGif(base64Data);
      } catch (error) {
        console.error(error);
        setErrorMessage("Failed to decode the GIF.");
      }
    } else {
      setErrorMessage("No file selected.");
    }
  };

  async function getBase64(path: string) {
    const base64Data: string = await invoke('get_base64', { path });
    return base64Data;
  }

  return (
    <div className="container">
      <div>
        <p>NEW BUTTON</p>
        <button type="button" onClick={() => getGif()}> READ FILE CONTENTS </button>
      </div>
      {imgSrc && (
        <div>
          <p>Decoded GIF:</p>
          <img src={imgSrc} alt="Decoded GIF" />
        </div>
      )}
      {errorMessage && (
        <div>
          <p>Error:</p>
          <p>{errorMessage}</p>
        </div>
      )}
    </div>
  );
}

export default App;
