const Display = (path: string) => {
    console.log("path before");
    console.log(path); 
    const x = JSON.stringify(path);
    return (
        <div>
            <p>path: {x}</p>
            <img src={x} alt="secondaryImage" />
        </div>
    );
}
export default Display;