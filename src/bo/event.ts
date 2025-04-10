import {LogLevel} from "./common.ts";

export class DownloadSpeedInfoEvent {
    private _bytesPerSecond: number
    get bytesPerSecond(): number {
        return this._bytesPerSecond;
    }

    set bytesPerSecond(value: number) {
        this._bytesPerSecond = value;
    }
}

export class UploadSpeedInfoEvent {
    private _bytesPerSecond: number
    get bytesPerSecond(): number {
        return this._bytesPerSecond;
    }

    set bytesPerSecond(value: number) {
        this._bytesPerSecond = value;
    }
}

class LogInfoEvent {
    private _level: LogLevel;
    get level(): LogLevel {
        return this._level;
    }

    set level(value: LogLevel) {
        this._level = value;
    }

    private _timestamp: Date;
    get timestamp(): Date {
        return this._timestamp;
    }

    set timestamp(value: Date) {
        this._timestamp = value;
    }

    private _message: string;
    get message(): string {
        return this._message;
    }

    set message(value: string) {
        this._message = value;
    }
}