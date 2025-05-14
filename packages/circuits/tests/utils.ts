export function bytesToChunkedFields(bytes: number[], bitsPerChunk: number, numChunks: number): string[] {
    // Convert bytes array to BigInt (big-endian)
    let num = BigInt(0);
    for (const byte of bytes) {
        num = (num << BigInt(8)) + BigInt(byte);
    }
    
    // Chunk the BigInt following the Rust implementation
    const chunks: string[] = [];
    const chunkSize = BigInt(2) ** BigInt(bitsPerChunk);
    
    for (let i = 0; i < numChunks; i++) {
        const chunk = num % chunkSize;
        num = num >> BigInt(bitsPerChunk);
        chunks.push(chunk.toString());
    }
    
    return chunks;
}