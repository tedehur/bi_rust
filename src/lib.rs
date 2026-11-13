//! Démonstrateur de trame minimal pour le dépôt bi_rust.
//!
//! Ce n'est pas encore le protocole « Double Flag » annoncé dans les échanges,
//! ni un codec de compression ou un protocole radio prêt pour la production.
//! Le format ci-dessous sert uniquement à rendre explicite un premier contrat
//! logiciel testable : version, catégorie de message, identifiant de symbole.

/// Version de la trame de démonstration.
pub const FRAME_VERSION: u8 = 1;
/// Taille fixe d'une trame de démonstration, en octets.
pub const FRAME_LEN: usize = 3;

/// Catégorie de message illustrative. `Alert` ne confère aucune autorité
/// d'alerte et n'est pas un niveau de sécurité.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageClass {
    Everyday = 0,
    Alert = 1,
}

/// Erreurs possibles lors du décodage de la trame de démonstration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    WrongLength,
    UnsupportedVersion,
    UnknownClass,
}

/// Trame conceptuelle : version, classe, identifiant de symbole.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame {
    pub class: MessageClass,
    pub symbol_id: u8,
}

impl Frame {
    /// Sérialise la trame de démonstration sur trois octets.
    pub const fn encode(self) -> [u8; FRAME_LEN] {
        [FRAME_VERSION, self.class as u8, self.symbol_id]
    }

    /// Décode strictement trois octets. L'intégrité, l'authenticité,
    /// l'anti-rejeu et le transport radio ne sont pas traités ici.
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        if bytes.len() != FRAME_LEN {
            return Err(DecodeError::WrongLength);
        }
        if bytes[0] != FRAME_VERSION {
            return Err(DecodeError::UnsupportedVersion);
        }
        let class = match bytes[1] {
            0 => MessageClass::Everyday,
            1 => MessageClass::Alert,
            _ => return Err(DecodeError::UnknownClass),
        };
        Ok(Self {
            class,
            symbol_id: bytes[2],
        })
    }
}

/// Interprétation statique d'un mot-clé BIBRAI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BibraiConcept {
    pub code: &'static str,
    pub pedagogy: &'static str,
    pub education: &'static str,
}

static BIBRAI_CONCEPTS: [BibraiConcept; 7] = [
    BibraiConcept {
        code: "FEU",
        pedagogy: "Incendie",
        education: "Incendie",
    },
    BibraiConcept {
        code: "SAUVE",
        pedagogy: "Secours",
        education: "Secours",
    },
    BibraiConcept {
        code: "ALERTE",
        pedagogy: "Attention",
        education: "Attention",
    },
    BibraiConcept {
        code: "EVACUER",
        pedagogy: "Sortir",
        education: "Evacuation",
    },
    BibraiConcept {
        code: "URGENCE",
        pedagogy: "Probleme",
        education: "Urgence",
    },
    BibraiConcept {
        code: "SECURITE",
        pedagogy: "Securite",
        education: "Securite",
    },
    BibraiConcept {
        code: "FAIL-SAFE",
        pedagogy: "Secours",
        education: "Mode secours",
    },
];

/// Décode un mot-clé BIBRAI complet sans modifier le contenu reçu.
///
/// La comparaison ignore la casse ASCII et les espaces ASCII en bordure.
/// `ÉVACUER` et `éVACUER` sont également reconnus. Ce dictionnaire textuel
/// est indépendant du format binaire `Frame`.
pub fn decode_bibrai_text(payload: &[u8]) -> Option<&'static BibraiConcept> {
    BIBRAI_CONCEPTS
        .iter()
        .find(|concept| matches_bibrai_token(payload, concept.code.as_bytes()))
}

fn is_ascii_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C)
}

fn matches_bibrai_token(payload: &[u8], token: &[u8]) -> bool {
    let mut begin = 0;
    let mut end = payload.len();
    while begin < end && is_ascii_whitespace(payload[begin]) {
        begin += 1;
    }
    while end > begin && is_ascii_whitespace(payload[end - 1]) {
        end -= 1;
    }

    let mut token_index = 0;
    while begin < end && token_index < token.len() {
        let value = if payload[begin] == 0xC3 {
            if begin + 1 >= end || !matches!(payload[begin + 1], 0x89 | 0xA9) {
                return false;
            }
            begin += 2;
            b'E'
        } else {
            let value = payload[begin].to_ascii_uppercase();
            begin += 1;
            value
        };

        if value != token[token_index] {
            return false;
        }
        token_index += 1;
    }

    begin == end && token_index == token.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_everyday_message() {
        let frame = Frame {
            class: MessageClass::Everyday,
            symbol_id: 7,
        };
        assert_eq!(Frame::decode(&frame.encode()), Ok(frame));
    }

    #[test]
    fn round_trip_alert_class() {
        let frame = Frame {
            class: MessageClass::Alert,
            symbol_id: 42,
        };
        assert_eq!(Frame::decode(&frame.encode()), Ok(frame));
    }

    #[test]
    fn rejects_wrong_length_and_version() {
        assert_eq!(Frame::decode(&[1, 0]), Err(DecodeError::WrongLength));
        assert_eq!(
            Frame::decode(&[2, 0, 7]),
            Err(DecodeError::UnsupportedVersion)
        );
    }

    #[test]
    fn rejects_unknown_class() {
        assert_eq!(Frame::decode(&[1, 2, 7]), Err(DecodeError::UnknownClass));
    }

    #[test]
    fn decodes_all_bibrai_concepts() {
        for concept in &BIBRAI_CONCEPTS {
            assert_eq!(decode_bibrai_text(concept.code.as_bytes()), Some(concept));
        }
    }

    #[test]
    fn accepts_ascii_case_and_outer_whitespace() {
        assert_eq!(decode_bibrai_text(b" \tfeu\r\n"), Some(&BIBRAI_CONCEPTS[0]));
    }

    #[test]
    fn accepts_accented_e_for_evacuate_token() {
        assert_eq!(
            decode_bibrai_text(" \u{00E9}vacuer ".as_bytes()),
            Some(&BIBRAI_CONCEPTS[3])
        );
        assert_eq!(
            decode_bibrai_text("\u{00C9}VACUER".as_bytes()),
            Some(&BIBRAI_CONCEPTS[3])
        );
    }

    #[test]
    fn rejects_partial_or_invalid_bibrai_messages() {
        for message in [b"LE FEU".as_slice(), b"FEU!", b"FEUX", b"UNKNOWN", b""] {
            assert_eq!(decode_bibrai_text(message), None);
        }
        assert_eq!(decode_bibrai_text(&[0xFF]), None);
    }
}
