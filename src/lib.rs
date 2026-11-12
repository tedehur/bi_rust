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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_everyday_message() {
        let frame = Frame { class: MessageClass::Everyday, symbol_id: 7 };
        assert_eq!(Frame::decode(&frame.encode()), Ok(frame));
    }

    #[test]
    fn round_trip_alert_class() {
        let frame = Frame { class: MessageClass::Alert, symbol_id: 42 };
        assert_eq!(Frame::decode(&frame.encode()), Ok(frame));
    }

    #[test]
    fn rejects_wrong_length_and_version() {
        assert_eq!(Frame::decode(&[1, 0]), Err(DecodeError::WrongLength));
        assert_eq!(Frame::decode(&[2, 0, 7]), Err(DecodeError::UnsupportedVersion));
    }

    #[test]
    fn rejects_unknown_class() {
        assert_eq!(Frame::decode(&[1, 2, 7]), Err(DecodeError::UnknownClass));
    }
}
