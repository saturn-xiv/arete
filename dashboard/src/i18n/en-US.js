export default {
    languages: {
        'en-US': 'English',
        'zh-Hans': '简体中文',
        'zh-Hant': '繁體中文',
    },
    buttons: {
        submit: 'Submit',
        actions: 'Actions',
        close: 'Close',
    },
    flashes: {
        success: "Success"
    },
    form: {
        labels: {
            password: 'Password',
            'password-confirmation': 'Password confirmation',
            'nickname': 'Nickname',
            'real-name': 'Real name',
            'new-password': 'New password',
            'current-password': 'Current password',
            email: 'Email',
            logo: 'Logo',
            'created-at': 'Created at',
            'updated-at': 'Updated at',
            message: 'Message',
            ip: 'IP',
            id: 'ID',
            host: 'Host',
            port: 'Port',
            startup: 'Startup',
            shutdown: 'Shutdown',
            enable: 'Enable',
            yes: 'Y',
            no: 'N',
            true: 'T',
            false: 'F',
            range: {
                date: 'Date range',
                time: 'Time range',
                timestamp: 'Timestamp range'
            },
            file: {
                path: 'Path',
                mode: 'Mode',
                content: 'Content'
            }
        }
    },
    nut: {
        install: {
            title: 'Install'
        },
        personal: {
            title: 'Personal'
        },
        users: {
            'sign-in': {
                title: 'Sign in',
                login: 'Emil/Nickname'
            },
            'sign-up': {
                title: 'Sign up'
            },
            'profile': {
                title: 'Profile'
            },
            'logs': {
                title: 'Logs'
            },
            'change-password': {
                title: 'Change password'
            }
        }
    },
    ops: {
        vpn: {
            form: {
                labels: {
                    log: {
                        remote: 'Remove',
                        trusted: 'Trusted',
                        received: 'Received',
                        send: 'Send',
                        'opened-at': 'Opened at',
                        'closed-at': 'Closed at',
                    },
                    user: {
                        online: 'Online'
                    },
                }
            },
            dashboard: {
                title: "VPN",
                files: 'OpenVpn config files'
            },
            users: {
                index: {
                    title: 'Users',
                },
                new: {
                    title: 'Create a new user'
                },
                edit: {
                    title: 'Edit user {name}'
                },
                'change-password': {
                    title: 'Change vpn password'
                }
            },
            settings: {
                title: 'Settings',
                host: 'Domain/Public ip address',
                interface: 'Network device',
                ip: 'Listen ip address',
                dns: 'Fallback DNS server',
                server: {
                    netmask: 'Local netmask',
                    network: 'Local network',
                },
                client: {
                    netmask: 'VPN netmask',
                    network: 'VPN network',
                }
            },
            logs: {
                index: {
                    title: 'Logs'
                }
            }
        }
    }
}