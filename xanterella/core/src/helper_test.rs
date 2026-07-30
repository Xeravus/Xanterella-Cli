use crate::helper::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_installer_helper_get_sshstring() {
        let mut xanterella1 = Xanterella::new();
        let mut xanterella2 = Xanterella::new();
        let mut xanterella3 = Xanterella::new();
        let mut xanterella4 = Xanterella::new();
        let mut xanterella5 = Xanterella::new();

        let mut install1 = XanterellaInstall::new(&mut xanterella1);
        let mut install2 = XanterellaInstall::new(&mut xanterella2);
        let mut install3 = XanterellaInstall::new(&mut xanterella3);
        let mut install4 = XanterellaInstall::new(&mut xanterella4);
        let mut install5 = XanterellaInstall::new(&mut xanterella5);

        install1.ip = "127.0.0.1".to_string();
        install2.ip = "127.0.0.1".to_string();

        install3.ip = "test".to_string();
        install4.ip = "test".to_string();

        let result1 = install1.get_sshstring(User::Root);
        let result2 = install2.get_sshstring(User::Cato);
        let result3 = install3.get_sshstring(User::Root);
        let result4 = install4.get_sshstring(User::Cato);
        let result5 = install5.get_sshstring(User::Root);

        assert!(!result1.is_empty());
        assert!(!result2.is_empty());
        assert!(!result3.is_empty());
        assert!(!result4.is_empty());
        assert!(!result5.is_empty());

        assert_eq!(result1[0], "-o".to_string());
        assert_eq!(result2[0], "-o".to_string());
        assert_eq!(result3[0], "-o".to_string());
        assert_eq!(result4[0], "-o".to_string());
        assert_eq!(result5[0], "-o".to_string());

        assert_eq!(result1[1], "StrictHostKeyChecking=no".to_string());
        assert_eq!(result2[1], "StrictHostKeyChecking=no".to_string());
        assert_eq!(result3[1], "StrictHostKeyChecking=no".to_string());
        assert_eq!(result4[1], "StrictHostKeyChecking=no".to_string());
        assert_eq!(result5[1], "StrictHostKeyChecking=no".to_string());

        assert_eq!(result1[2], "-o".to_string());
        assert_eq!(result2[2], "-o".to_string());
        assert_eq!(result3[2], "-o".to_string());
        assert_eq!(result4[2], "-o".to_string());
        assert_eq!(result5[2], "-o".to_string());

        assert_eq!(result1[3], "UserKnownHostsFile=/dev/null".to_string());
        assert_eq!(result2[3], "UserKnownHostsFile=/dev/null".to_string());
        assert_eq!(result3[3], "UserKnownHostsFile=/dev/null".to_string());
        assert_eq!(result4[3], "UserKnownHostsFile=/dev/null".to_string());
        assert_eq!(result5[3], "UserKnownHostsFile=/dev/null".to_string());

        assert_eq!(result1[4], "root@127.0.0.1".to_string());
        assert_eq!(result2[4], "cato@127.0.0.1".to_string());
        assert_eq!(result3[4], "root@test".to_string());
        assert_eq!(result4[4], "cato@test".to_string());
        assert_eq!(result5[4], "root@".to_string());
    }

    #[test]
    fn test_installer_helper_get_hardware() {
        let mut xanterella = Xanterella::new();
        let mut install = XanterellaInstall::new(&mut xanterella);

        let result = install.get_hardware();

        assert!(result.is_err());
    }
}
