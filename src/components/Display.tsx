const Display = (path: string) => {
    const x = JSON.stringify(path);
    return (
        <div>
            <p>path: {x}</p>
            <img src={path} alt="secondaryImage" />
        </div>
    );
}
export default Display;