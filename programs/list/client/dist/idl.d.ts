export declare type ListProgram = {
    "version": "0.0.3";
    "name": "list_program";
    "instructions": [
        {
            "name": "createList";
            "accounts": [
                {
                    "name": "list";
                    "isMut": true;
                    "isSigner": false;
                },
                {
                    "name": "namespace";
                    "isMut": false;
                    "isSigner": false;
                },
                {
                    "name": "owner";
                    "isMut": true;
                    "isSigner": true;
                },
                {
                    "name": "payer";
                    "isMut": true;
                    "isSigner": true;
                },
                {
                    "name": "systemProgram";
                    "isMut": false;
                    "isSigner": false;
                }
            ];
            "args": [
                {
                    "name": "bump";
                    "type": "u8";
                }
            ];
        },
        {
            "name": "deleteList";
            "accounts": [
                {
                    "name": "list";
                    "isMut": true;
                    "isSigner": false;
                },
                {
                    "name": "owner";
                    "isMut": true;
                    "isSigner": true;
                }
            ];
            "args": [];
        },
        {
            "name": "popElement";
            "accounts": [
                {
                    "name": "list";
                    "isMut": true;
                    "isSigner": false;
                },
                {
                    "name": "owner";
                    "isMut": true;
                    "isSigner": true;
                },
                {
                    "name": "element";
                    "isMut": true;
                    "isSigner": false;
                }
            ];
            "args": [];
        },
        {
            "name": "pushElement";
            "accounts": [
                {
                    "name": "list";
                    "isMut": true;
                    "isSigner": false;
                },
                {
                    "name": "owner";
                    "isMut": true;
                    "isSigner": true;
                },
                {
                    "name": "payer";
                    "isMut": true;
                    "isSigner": true;
                },
                {
                    "name": "element";
                    "isMut": true;
                    "isSigner": false;
                },
                {
                    "name": "systemProgram";
                    "isMut": false;
                    "isSigner": false;
                }
            ];
            "args": [
                {
                    "name": "value";
                    "type": "publicKey";
                },
                {
                    "name": "bump";
                    "type": "u8";
                }
            ];
        }
    ];
    "accounts": [
        {
            "name": "element";
            "type": {
                "kind": "struct";
                "fields": [
                    {
                        "name": "index";
                        "type": "u128";
                    },
                    {
                        "name": "value";
                        "type": "publicKey";
                    },
                    {
                        "name": "bump";
                        "type": "u8";
                    }
                ];
            };
        },
        {
            "name": "list";
            "type": {
                "kind": "struct";
                "fields": [
                    {
                        "name": "owner";
                        "type": "publicKey";
                    },
                    {
                        "name": "namespace";
                        "type": "publicKey";
                    },
                    {
                        "name": "count";
                        "type": "u128";
                    },
                    {
                        "name": "bump";
                        "type": "u8";
                    }
                ];
            };
        }
    ];
};
export declare const IDL: ListProgram;
