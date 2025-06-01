export function base64ToArrayBuffer(base64: string): ArrayBuffer {
    // Replace URL-safe characters
    base64 = base64.replace(/-/g, '+').replace(/_/g, '/');

    // Re-add base64 padding
    while (base64.length % 4 !== 0) {
        base64 += '=';
    }

    // From here, just normal base64 deserialization
    // (shamelessly stolen from https://stackoverflow.com/a/21797381)
    // Also pretty pathetic that there is no single function for this is js
    const binaryString = atob(base64);
    const bytes = new Uint8Array(binaryString.length);

    for (let i = 0; i < binaryString.length; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }

    return bytes.buffer;
}