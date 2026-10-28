// Developer notes
//
// It seems that creating the logic written in the logic diagram requires:
// 1. Type erasure machinery
// 2. Regular based machinery
//
// This hints that the logic diagram is missing one more step,
// say an information flow diagram.
//
// Architecture diagram is not required as the software sense is built
// on top of known assumption that the architecture limits what could be done
// by the software.
//
// However, after working on this, it is apparent that the information diagram
// holds something that architecture diagram doesn't explain, including:
// 1. Type erasure machinery represented as union of counterfactual
// 2. Evolution of existing representation into to contain more information
//
// We don't want those to be hardcoded as that would require revisiting the code
// multiple times, and seemingly behaving like a naive implementation of the logic
//
// So, what is an "information diagram"? It describes how information flows
// from one to another. It also describes information group, which bundles
// a unit that generate information, or multiples of one segment, that together
// work to form the correct information graph.

pub mod registry {
    pub struct IdentifierTracker {}
    pub struct Identifier {}
    pub struct RegistryMap {}
    pub struct Registry {}
}

pub mod runtime {}

pub mod unit {
    pub struct Hub {
        members: Vec<Member>,
    }

    pub struct Member {}
}

pub mod issue {
    use crate::{document, registry, unit};

    pub struct Generator {}

    pub struct Document {
        identifier: Identifier,
        internal_message: InternalMessage,
        external_form: ExernalForm,
    }

    pub struct Identifier {
        registry: registry::Registry,
        identifier: registry::Identifier,
    }

    pub struct Authorization {
        authors: Vec<unit::Member>,
    }

    pub struct Provenance {
        authors: Vec<Identifier>,
    }

    pub struct InternalMessage {
        registry: registry::Registry,
        begin_payload: document::Field,
        authorization: Authorization,
        provenance: Provenance,
    }

    pub struct ExernalForm {}
}

pub mod document {
    use crate::registry;
    pub struct Field {
        field: Vec<FieldPair>,
    }
    pub struct FieldPair {
        field_name: String,
        field_type: registry::Registry,
    }
}
