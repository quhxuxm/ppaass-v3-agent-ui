export class UserInfo {
    private _proxyAddress: string;

    get proxyAddress(): string {
        return this._proxyAddress;
    }

    set proxyAddress(value: string) {
        this._proxyAddress = value;
    }

    private _userName: string;

    get userName(): string {
        return this._userName;
    }

    set userName(value: string) {
        this._userName = value;
    }
}