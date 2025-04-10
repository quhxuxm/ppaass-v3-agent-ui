import {LogLevel} from "./common.ts";

class ConnectionPoolConfiguration {
    private _check_interval: number;
    get check_interval(): number {
        return this._check_interval;
    }

    set check_interval(value: number) {
        this._check_interval = value;
    }

    private _fill_interval: number;
    get fill_interval(): number {
        return this._fill_interval;
    }

    set fill_interval(value: number) {
        this._fill_interval = value;
    }

    private _maxPoolSize: number;
    get maxPoolSize(): number {
        return this._maxPoolSize;
    }

    set maxPoolSize(value: number) {
        this._maxPoolSize = value;
    }
}

export class Configuration {
    private _serverPort: number;
    get serverPort(): number {
        return this._serverPort;
    }

    set serverPort(value: number) {
        this._serverPort = value;
    }

    private _workerThreadNumber: number;
    get workerThreadNumber(): number {
        return this._workerThreadNumber;
    }

    set workerThreadNumber(value: number) {
        this._workerThreadNumber = value;
    }

    private _maxLogLevel: LogLevel
    get maxLogLevel(): LogLevel {
        return this._maxLogLevel;
    }

    set maxLogLevel(value: LogLevel) {
        this._maxLogLevel = value;
    }

    private _connectionPoolConfiguration: ConnectionPoolConfiguration
    get connectionPoolConfiguration(): ConnectionPoolConfiguration {
        return this._connectionPoolConfiguration;
    }

    set connectionPoolConfiguration(value: ConnectionPoolConfiguration) {
        this._connectionPoolConfiguration = value;
    }
}