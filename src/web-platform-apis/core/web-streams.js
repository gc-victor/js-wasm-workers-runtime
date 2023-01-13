import {
    ReadableStream,
    ReadableStreamDefaultController,
    ReadableByteStreamController,
    ReadableStreamBYOBRequest,
    ReadableStreamDefaultReader,
    ReadableStreamBYOBReader,
    WritableStream,
    WritableStreamDefaultController,
    WritableStreamDefaultWriter,
    ByteLengthQueuingStrategy,
    CountQueuingStrategy,
    TransformStream,
    TransformStreamDefaultController,
} from "web-streams-polyfill";

globalThis.ReadableStream = ReadableStream;
globalThis.ReadableStreamDefaultController = ReadableStreamDefaultController;
globalThis.ReadableByteStreamController = ReadableByteStreamController;
globalThis.ReadableStreamBYOBRequest = ReadableStreamBYOBRequest;
globalThis.ReadableStreamDefaultReader = ReadableStreamDefaultReader;
globalThis.ReadableStreamBYOBReader = ReadableStreamBYOBReader;

globalThis.WritableStream = WritableStream;
globalThis.WritableStreamDefaultController = WritableStreamDefaultController;
globalThis.WritableStreamDefaultWriter = WritableStreamDefaultWriter;

globalThis.ByteLengthQueuingStrategy = ByteLengthQueuingStrategy;
globalThis.CountQueuingStrategy = CountQueuingStrategy;

globalThis.TransformStream = TransformStream;
globalThis.TransformStreamDefaultController = TransformStreamDefaultController;
