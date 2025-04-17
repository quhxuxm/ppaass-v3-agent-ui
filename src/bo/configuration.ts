import {LogLevel} from "./common.ts";

export class ConnectionPoolConfiguration {
    constructor(checkInterval: number, fillInterval: number, maxPoolSize: number) {
        this._checkInterval = checkInterval;
        this._fillInterval = fillInterval;
        this._maxPoolSize = maxPoolSize;
    }

    private _checkInterval?: number;
    get checkInterval(): number | undefined {
        return this._checkInterval;
    }

    set checkInterval(value: number) {
        this._checkInterval = value;
    }

    private _fillInterval?: number;
    get fillInterval(): number | undefined {
        return this._fillInterval;
    }

    set fillInterval(value: number) {
        this._fillInterval = value;
    }

    private _maxPoolSize?: number;
    get maxPoolSize(): number | undefined {
        return this._maxPoolSize;
    }

    set maxPoolSize(value: number) {
        this._maxPoolSize = value;
    }
}

export class Configuration {
    constructor(serverPort: number, workerThreadNumber: number, maxLogLevel: LogLevel, connectionPoolConfiguration: ConnectionPoolConfiguration) {
        this._serverPort = serverPort;
        this._workerThreadNumber = workerThreadNumber;
        this._maxLogLevel = maxLogLevel;
        this._connectionPoolConfiguration = connectionPoolConfiguration;
    }

    private _serverPort?: number;
    get serverPort(): number | undefined {
        return this._serverPort;
    }

    set serverPort(value: number) {
        this._serverPort = value;
    }

    private _workerThreadNumber?: number;
    get workerThreadNumber(): number | undefined {
        return this._workerThreadNumber;
    }

    set workerThreadNumber(value: number) {
        this._workerThreadNumber = value;
    }

    private _maxLogLevel?: LogLevel
    get maxLogLevel(): LogLevel | undefined {
        return this._maxLogLevel;
    }

    set maxLogLevel(value: LogLevel) {
        this._maxLogLevel = value;
    }

    private _connectionPoolConfiguration?: ConnectionPoolConfiguration
    get connectionPoolConfiguration(): ConnectionPoolConfiguration | undefined {
        return this._connectionPoolConfiguration;
    }

    set connectionPoolConfiguration(value: ConnectionPoolConfiguration) {
        this._connectionPoolConfiguration = value;
    }
}