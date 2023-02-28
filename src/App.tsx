import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/dialog';
import { readTextFile } from '@tauri-apps/api/fs';

import './styles/App.css';

function App() {

  const [pathMsg, setPath] = useState("");                    // path to app
  const [defaultImage, getImage] = useState("");              // absolute path to image
  const [secondaryImage, getSecondaryImage] = useState("");   // relative path to image

  const readFileContents = async () => {
    try {
      const selectedPath = await open({multiple: false, title: 'Open Text File'});
      console.log(selectedPath);
      if(!selectedPath) return;
      const testImg = require(selectedPath as string);
      return <img src={testImg}></img>
    } catch (error) {
      console.log(error);
    }
  };

  async function get_path() {
    setPath(await invoke("path"));
  }

  async function get_default_image() {
    getImage(await invoke("default_image"));
  }
  
  async function get_relative_path_image() {
    getSecondaryImage(await invoke("relative_path_image"));

  }

  return (
    <div className="container">
{/*       <Display path={secondaryImage} /> */}
      <div className="row">
        <div>
          <button type="button" onClick={() => get_path()}>
            GET PATH
          </button>
          <button type="button" onClick={() => get_default_image()}>
            GET DEFAULT IMAGE
          </button>
          <button type="button" onClick={() => get_relative_path_image()}>
            GET SECONDARY IMAGE
          </button>
        </div>
      </div>
      <div>
        <p>pathMsg</p>
        <p>{pathMsg}</p>
      </div>
      <div>
        <p>NEW BUTTON</p>
        <button type="button" onClick={() => readFileContents()}> READ FILE CONTENTS </button>
      </div>
      <div>
        <p>default Image</p>
        <p>{defaultImage}</p>
        <img src={defaultImage} />
      </div>
      <div>
        <p>secondaryImage</p>
        <p>{secondaryImage}</p>
        <img src={secondaryImage} />
      </div>
    </div>
  );
}

export default App;
