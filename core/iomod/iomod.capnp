@0xdefbefb7e7579c48;

interface Agent {
    invoke @0 (coordinates: Text) -> (result: Data);
}

interface Iomod {
    getDeclaration @0 () -> (decl: Declaration);

    invoke @1 (coordinates: Text) -> (result: Data);

    struct Declaration {
        name         @0: Text;
        namespace    @1: Text;
        organization @2: Text;

        calls @3: List(Call);
    }

    struct Call {
        name @0: Text;
    }
}

interface Registry {
    register @0 (coordinates: Text, iomod: Iomod);
}
