@0xdefbefb7e7579c48;

interface Agent {
    invoke @0 (path: Text);
}

interface Iomod {
    getDeclaration @0 () -> (decl: Declaration);

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
    register @0 (coordinates: Text, iomod: Iomod) -> (agent: Agent);
}
