class fileDropEvent {
    event: string;
    windowLabel: string;
    payload: string[];
    id: number;

    constructor(event: string, windowLabel: string, payload: string[], id: number) {
        this.event = event;
        this.windowLabel = windowLabel;
        this.payload = payload;
        this.id = id;
    }
}
