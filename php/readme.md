`protoc --php_out=specs cpu.proto`

// if response is not 200, the api might return a different message 

    // message ErrorResponse {
    //     ErrorEnum code = 1;        // e.g., HTTP or app-specific code
    //     string message = 2;    // Human-readable
    //     repeated string details = 3; // Optional extra info
    // }
    // enum ErrorEnum {
    //     NO_RESULTS = 0;
    //     UNKNOWN = 7;
    // }