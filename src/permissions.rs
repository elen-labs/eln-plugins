//! Tool 호출 권한 비트 모음.
//!
//! `ADMIN`은 별도 비트 자리 없이 `READ | WRITE | MODIFY` OR alias.
//! 새 권한 비트를 추가하면 `ADMIN` 정의를 명시적으로 갱신할 것 — 권한 표면의
//! 묵시적 확장을 막기 위함.
//!
//! S2: transport-level grant derivation (ApiKey → Permissions)은 S3 scope.
//! 현재 stdio 진입은 `Permissions::ADMIN`을 hard-code로 부여, HTTP는 `READ`만.

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Permissions: u32 {
        const READ   = 1 << 0;
        const WRITE  = 1 << 1;
        const MODIFY = 1 << 2;
    }
}

impl Permissions {
    pub const ADMIN: Self = Self::READ
        .union(Self::WRITE)
        .union(Self::MODIFY);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_is_read_write_modify() {
        assert!(Permissions::ADMIN.contains(Permissions::READ));
        assert!(Permissions::ADMIN.contains(Permissions::WRITE));
        assert!(Permissions::ADMIN.contains(Permissions::MODIFY));
        assert_eq!(
            Permissions::ADMIN,
            Permissions::READ | Permissions::WRITE | Permissions::MODIFY
        );
    }

    #[test]
    fn empty_grants_nothing() {
        let p = Permissions::empty();
        assert!(!p.contains(Permissions::READ));
        assert!(!p.contains(Permissions::WRITE));
        assert!(!p.contains(Permissions::MODIFY));
    }
}
