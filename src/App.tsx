import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

import './styles/App.css';
import Display from './components/Display';
import gif from './assets/images/gif/fire.gif';

function App() {

  const [pathMsg, setPath] = useState("");                  // path to app
  const [defaultImage, getImage] = useState("");            // absolute path to image
  const [secondaryImage, getSecondaryImage] = useState(""); // relative path to image

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
      <Display path={secondaryImage}></Display>
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
        <p>{pathMsg}</p>
      </div>
      <div>
        <p>default Image</p>
        <p>{defaultImage}</p>
        <img src={defaultImage} />
      </div>
      <div>
        <p>Relative path</p>
        <p>{secondaryImage}</p>
        <img src={secondaryImage} />
      </div>
      <div>
        <p>Safe test</p>
        <img src={gif} />
      </div>
    </div>
  );
}

export default App;
