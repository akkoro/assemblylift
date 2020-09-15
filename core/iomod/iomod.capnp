@0xdefbefb7e7579c48;

interface Agent {
    invoke @0 (coordinates: Text, input: Data) -> (result: Data);
}

interface Iomod {
    invoke @0 (coordinates: Text, input: Data) -> (result: Data);
}

interface Registry {
    register @0 (coordinates: Text, iomod: Iomod);
}
