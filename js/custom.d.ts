export interface Core {
    getPeripheral(id: string): any,
    getPeripheralType(id: string): string,

    getPeripheralsIds(): string[],
    onPeripheralMounted?: (id: string) => void,
    onPeripheralUnMounted?: (id: string) => void,
}