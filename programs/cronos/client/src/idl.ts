export type Cronos = {
  version: "0.0.1";
  name: "cronos";
  instructions: [
    {
      name: "configUpdateAdminAuthority";
      accounts: [
        {
          name: "admin";
          isMut: true;
          isSigner: true;
        },
        {
          name: "config";
          isMut: true;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "newAdminAuthority";
          type: "publicKey";
        }
      ];
    },
    {
      name: "configUpdateFrameInterval";
      accounts: [
        {
          name: "admin";
          isMut: true;
          isSigner: true;
        },
        {
          name: "config";
          isMut: true;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "newFrameInterval";
          type: "u64";
        }
      ];
    },
    {
      name: "configUpdateProgramFee";
      accounts: [
        {
          name: "admin";
          isMut: true;
          isSigner: true;
        },
        {
          name: "config";
          isMut: true;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "newProgramFee";
          type: "u64";
        }
      ];
    },
    {
      name: "configUpdateWorkerFee";
      accounts: [
        {
          name: "admin";
          isMut: true;
          isSigner: true;
        },
        {
          name: "config";
          isMut: true;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "newWorkerFee";
          type: "u64";
        }
      ];
    },
    {
      name: "daemonCreate";
      accounts: [
        {
          name: "daemon";
          isMut: true;
          isSigner: false;
        },
        {
          name: "owner";
          isMut: true;
          isSigner: true;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "bump";
          type: "u8";
        }
      ];
    },
    {
      name: "daemonInvoke";
      accounts: [
        {
          name: "daemon";
          isMut: false;
          isSigner: false;
        },
        {
          name: "owner";
          isMut: true;
          isSigner: true;
        }
      ];
      args: [
        {
          name: "instructionData";
          type: {
            defined: "InstructionData";
          };
        }
      ];
    },
    {
      name: "frameCreate";
      accounts: [
        {
          name: "authority";
          isMut: true;
          isSigner: false;
        },
        {
          name: "clock";
          isMut: false;
          isSigner: false;
        },
        {
          name: "config";
          isMut: false;
          isSigner: false;
        },
        {
          name: "frame";
          isMut: true;
          isSigner: false;
        },
        {
          name: "indexerProgram";
          isMut: false;
          isSigner: false;
        },
        {
          name: "list";
          isMut: true;
          isSigner: false;
        },
        {
          name: "payer";
          isMut: true;
          isSigner: true;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "timestamp";
          type: "u64";
        },
        {
          name: "frameBump";
          type: "u8";
        },
        {
          name: "listBump";
          type: "u8";
        }
      ];
    },
    {
      name: "initialize";
      accounts: [
        {
          name: "authority";
          isMut: true;
          isSigner: false;
        },
        {
          name: "config";
          isMut: true;
          isSigner: false;
        },
        {
          name: "signer";
          isMut: true;
          isSigner: true;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        },
        {
          name: "treasury";
          isMut: true;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "authorityBump";
          type: "u8";
        },
        {
          name: "configBump";
          type: "u8";
        },
        {
          name: "treasuryBump";
          type: "u8";
        }
      ];
    },
    {
      name: "revenueCollect";
      accounts: [
        {
          name: "revenue";
          isMut: true;
          isSigner: false;
        },
        {
          name: "signer";
          isMut: true;
          isSigner: true;
        },
        {
          name: "treasury";
          isMut: true;
          isSigner: false;
        }
      ];
      args: [];
    },
    {
      name: "revenueCreate";
      accounts: [
        {
          name: "daemon";
          isMut: false;
          isSigner: false;
        },
        {
          name: "revenue";
          isMut: true;
          isSigner: false;
        },
        {
          name: "signer";
          isMut: true;
          isSigner: true;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "bump";
          type: "u8";
        }
      ];
    },
    {
      name: "taskCreate";
      accounts: [
        {
          name: "authority";
          isMut: true;
          isSigner: false;
        },
        {
          name: "daemon";
          isMut: true;
          isSigner: false;
        },
        {
          name: "frame";
          isMut: false;
          isSigner: false;
        },
        {
          name: "indexerProgram";
          isMut: false;
          isSigner: false;
        },
        {
          name: "task";
          isMut: true;
          isSigner: false;
        },
        {
          name: "taskElement";
          isMut: true;
          isSigner: false;
        },
        {
          name: "taskList";
          isMut: true;
          isSigner: false;
        },
        {
          name: "owner";
          isMut: true;
          isSigner: true;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        }
      ];
      args: [
        {
          name: "instructionData";
          type: {
            defined: "InstructionData";
          };
        },
        {
          name: "executeAt";
          type: "u64";
        },
        {
          name: "repeatEvery";
          type: "u64";
        },
        {
          name: "repeatUntil";
          type: "u64";
        },
        {
          name: "taskBump";
          type: "u8";
        },
        {
          name: "taskElementBump";
          type: "u8";
        }
      ];
    },
    {
      name: "taskExecute";
      accounts: [
        {
          name: "clock";
          isMut: false;
          isSigner: false;
        },
        {
          name: "config";
          isMut: false;
          isSigner: false;
        },
        {
          name: "daemon";
          isMut: true;
          isSigner: false;
        },
        {
          name: "revenue";
          isMut: true;
          isSigner: false;
        },
        {
          name: "task";
          isMut: true;
          isSigner: false;
        },
        {
          name: "worker";
          isMut: true;
          isSigner: true;
        }
      ];
      args: [];
    },
    {
      name: "taskRepeat";
      accounts: [
        {
          name: "authority";
          isMut: true;
          isSigner: false;
        },
        {
          name: "config";
          isMut: false;
          isSigner: false;
        },
        {
          name: "daemon";
          isMut: true;
          isSigner: false;
        },
        {
          name: "indexerProgram";
          isMut: false;
          isSigner: false;
        },
        {
          name: "nextFrame";
          isMut: false;
          isSigner: false;
        },
        {
          name: "nextTask";
          isMut: true;
          isSigner: false;
        },
        {
          name: "nextTaskElement";
          isMut: true;
          isSigner: false;
        },
        {
          name: "nextTaskList";
          isMut: true;
          isSigner: false;
        },
        {
          name: "prevTask";
          isMut: true;
          isSigner: false;
        },
        {
          name: "systemProgram";
          isMut: false;
          isSigner: false;
        },
        {
          name: "worker";
          isMut: false;
          isSigner: true;
        }
      ];
      args: [
        {
          name: "nextTaskBump";
          type: "u8";
        },
        {
          name: "nextTaskElementBump";
          type: "u8";
        }
      ];
    }
  ];
  accounts: [
    {
      name: "authority";
      type: {
        kind: "struct";
        fields: [
          {
            name: "bump";
            type: "u8";
          }
        ];
      };
    },
    {
      name: "config";
      type: {
        kind: "struct";
        fields: [
          {
            name: "adminAuthority";
            type: "publicKey";
          },
          {
            name: "frameInterval";
            type: "u64";
          },
          {
            name: "programFee";
            type: "u64";
          },
          {
            name: "workerFee";
            type: "u64";
          },
          {
            name: "bump";
            type: "u8";
          }
        ];
      };
    },
    {
      name: "daemon";
      type: {
        kind: "struct";
        fields: [
          {
            name: "owner";
            type: "publicKey";
          },
          {
            name: "totalTaskCount";
            type: "u128";
          },
          {
            name: "executedTaskCount";
            type: "u128";
          },
          {
            name: "bump";
            type: "u8";
          }
        ];
      };
    },
    {
      name: "frame";
      type: {
        kind: "struct";
        fields: [
          {
            name: "timestamp";
            type: "u64";
          },
          {
            name: "bump";
            type: "u8";
          }
        ];
      };
    },
    {
      name: "revenue";
      type: {
        kind: "struct";
        fields: [
          {
            name: "daemon";
            type: "publicKey";
          },
          {
            name: "balance";
            type: "u64";
          },
          {
            name: "bump";
            type: "u8";
          }
        ];
      };
    },
    {
      name: "task";
      type: {
        kind: "struct";
        fields: [
          {
            name: "daemon";
            type: "publicKey";
          },
          {
            name: "id";
            type: "u128";
          },
          {
            name: "instructionData";
            type: {
              defined: "InstructionData";
            };
          },
          {
            name: "status";
            type: {
              defined: "TaskStatus";
            };
          },
          {
            name: "executeAt";
            type: "u64";
          },
          {
            name: "repeatEvery";
            type: "u64";
          },
          {
            name: "repeatUntil";
            type: "u64";
          },
          {
            name: "bump";
            type: "u8";
          }
        ];
      };
    },
    {
      name: "treasury";
      type: {
        kind: "struct";
        fields: [
          {
            name: "bump";
            type: "u8";
          }
        ];
      };
    }
  ];
  types: [
    {
      name: "InstructionData";
      type: {
        kind: "struct";
        fields: [
          {
            name: "programId";
            type: "publicKey";
          },
          {
            name: "keys";
            type: {
              vec: {
                defined: "AccountMetaData";
              };
            };
          },
          {
            name: "data";
            type: "bytes";
          }
        ];
      };
    },
    {
      name: "AccountMetaData";
      type: {
        kind: "struct";
        fields: [
          {
            name: "pubkey";
            type: "publicKey";
          },
          {
            name: "isSigner";
            type: "bool";
          },
          {
            name: "isWritable";
            type: "bool";
          }
        ];
      };
    },
    {
      name: "TaskStatus";
      type: {
        kind: "enum";
        variants: [
          {
            name: "Done";
          },
          {
            name: "Pending";
          },
          {
            name: "Repeat";
          }
        ];
      };
    }
  ];
  errors: [
    {
      code: 6000;
      name: "InvalidSignatory";
      msg: "Your daemon cannot provide all required signatures for this instruction";
    },
    {
      code: 6001;
      name: "TaskNotPending";
      msg: "Task is not pending and may not executed";
    },
    {
      code: 6002;
      name: "TaskNotRepeatable";
      msg: "This task is not marked for repeat";
    },
    {
      code: 6003;
      name: "TaskNotDue";
      msg: "This task is not due and may not be executed yet";
    },
    {
      code: 6004;
      name: "Unknown";
      msg: "Unknown error";
    }
  ];
};

export const IDL: Cronos = {
  version: "0.0.1",
  name: "cronos",
  instructions: [
    {
      name: "configUpdateAdminAuthority",
      accounts: [
        {
          name: "admin",
          isMut: true,
          isSigner: true,
        },
        {
          name: "config",
          isMut: true,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "newAdminAuthority",
          type: "publicKey",
        },
      ],
    },
    {
      name: "configUpdateFrameInterval",
      accounts: [
        {
          name: "admin",
          isMut: true,
          isSigner: true,
        },
        {
          name: "config",
          isMut: true,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "newFrameInterval",
          type: "u64",
        },
      ],
    },
    {
      name: "configUpdateProgramFee",
      accounts: [
        {
          name: "admin",
          isMut: true,
          isSigner: true,
        },
        {
          name: "config",
          isMut: true,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "newProgramFee",
          type: "u64",
        },
      ],
    },
    {
      name: "configUpdateWorkerFee",
      accounts: [
        {
          name: "admin",
          isMut: true,
          isSigner: true,
        },
        {
          name: "config",
          isMut: true,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "newWorkerFee",
          type: "u64",
        },
      ],
    },
    {
      name: "daemonCreate",
      accounts: [
        {
          name: "daemon",
          isMut: true,
          isSigner: false,
        },
        {
          name: "owner",
          isMut: true,
          isSigner: true,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "bump",
          type: "u8",
        },
      ],
    },
    {
      name: "daemonInvoke",
      accounts: [
        {
          name: "daemon",
          isMut: false,
          isSigner: false,
        },
        {
          name: "owner",
          isMut: true,
          isSigner: true,
        },
      ],
      args: [
        {
          name: "instructionData",
          type: {
            defined: "InstructionData",
          },
        },
      ],
    },
    {
      name: "frameCreate",
      accounts: [
        {
          name: "authority",
          isMut: true,
          isSigner: false,
        },
        {
          name: "clock",
          isMut: false,
          isSigner: false,
        },
        {
          name: "config",
          isMut: false,
          isSigner: false,
        },
        {
          name: "frame",
          isMut: true,
          isSigner: false,
        },
        {
          name: "indexerProgram",
          isMut: false,
          isSigner: false,
        },
        {
          name: "list",
          isMut: true,
          isSigner: false,
        },
        {
          name: "payer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "timestamp",
          type: "u64",
        },
        {
          name: "frameBump",
          type: "u8",
        },
        {
          name: "listBump",
          type: "u8",
        },
      ],
    },
    {
      name: "initialize",
      accounts: [
        {
          name: "authority",
          isMut: true,
          isSigner: false,
        },
        {
          name: "config",
          isMut: true,
          isSigner: false,
        },
        {
          name: "signer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
        {
          name: "treasury",
          isMut: true,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "authorityBump",
          type: "u8",
        },
        {
          name: "configBump",
          type: "u8",
        },
        {
          name: "treasuryBump",
          type: "u8",
        },
      ],
    },
    {
      name: "revenueCollect",
      accounts: [
        {
          name: "revenue",
          isMut: true,
          isSigner: false,
        },
        {
          name: "signer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "treasury",
          isMut: true,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: "revenueCreate",
      accounts: [
        {
          name: "daemon",
          isMut: false,
          isSigner: false,
        },
        {
          name: "revenue",
          isMut: true,
          isSigner: false,
        },
        {
          name: "signer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "bump",
          type: "u8",
        },
      ],
    },
    {
      name: "taskCreate",
      accounts: [
        {
          name: "authority",
          isMut: true,
          isSigner: false,
        },
        {
          name: "daemon",
          isMut: true,
          isSigner: false,
        },
        {
          name: "frame",
          isMut: false,
          isSigner: false,
        },
        {
          name: "indexerProgram",
          isMut: false,
          isSigner: false,
        },
        {
          name: "task",
          isMut: true,
          isSigner: false,
        },
        {
          name: "taskElement",
          isMut: true,
          isSigner: false,
        },
        {
          name: "taskList",
          isMut: true,
          isSigner: false,
        },
        {
          name: "owner",
          isMut: true,
          isSigner: true,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "instructionData",
          type: {
            defined: "InstructionData",
          },
        },
        {
          name: "executeAt",
          type: "u64",
        },
        {
          name: "repeatEvery",
          type: "u64",
        },
        {
          name: "repeatUntil",
          type: "u64",
        },
        {
          name: "taskBump",
          type: "u8",
        },
        {
          name: "taskElementBump",
          type: "u8",
        },
      ],
    },
    {
      name: "taskExecute",
      accounts: [
        {
          name: "clock",
          isMut: false,
          isSigner: false,
        },
        {
          name: "config",
          isMut: false,
          isSigner: false,
        },
        {
          name: "daemon",
          isMut: true,
          isSigner: false,
        },
        {
          name: "revenue",
          isMut: true,
          isSigner: false,
        },
        {
          name: "task",
          isMut: true,
          isSigner: false,
        },
        {
          name: "worker",
          isMut: true,
          isSigner: true,
        },
      ],
      args: [],
    },
    {
      name: "taskRepeat",
      accounts: [
        {
          name: "authority",
          isMut: true,
          isSigner: false,
        },
        {
          name: "config",
          isMut: false,
          isSigner: false,
        },
        {
          name: "daemon",
          isMut: true,
          isSigner: false,
        },
        {
          name: "indexerProgram",
          isMut: false,
          isSigner: false,
        },
        {
          name: "nextFrame",
          isMut: false,
          isSigner: false,
        },
        {
          name: "nextTask",
          isMut: true,
          isSigner: false,
        },
        {
          name: "nextTaskElement",
          isMut: true,
          isSigner: false,
        },
        {
          name: "nextTaskList",
          isMut: true,
          isSigner: false,
        },
        {
          name: "prevTask",
          isMut: true,
          isSigner: false,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
        {
          name: "worker",
          isMut: false,
          isSigner: true,
        },
      ],
      args: [
        {
          name: "nextTaskBump",
          type: "u8",
        },
        {
          name: "nextTaskElementBump",
          type: "u8",
        },
      ],
    },
  ],
  accounts: [
    {
      name: "authority",
      type: {
        kind: "struct",
        fields: [
          {
            name: "bump",
            type: "u8",
          },
        ],
      },
    },
    {
      name: "config",
      type: {
        kind: "struct",
        fields: [
          {
            name: "adminAuthority",
            type: "publicKey",
          },
          {
            name: "frameInterval",
            type: "u64",
          },
          {
            name: "programFee",
            type: "u64",
          },
          {
            name: "workerFee",
            type: "u64",
          },
          {
            name: "bump",
            type: "u8",
          },
        ],
      },
    },
    {
      name: "daemon",
      type: {
        kind: "struct",
        fields: [
          {
            name: "owner",
            type: "publicKey",
          },
          {
            name: "totalTaskCount",
            type: "u128",
          },
          {
            name: "executedTaskCount",
            type: "u128",
          },
          {
            name: "bump",
            type: "u8",
          },
        ],
      },
    },
    {
      name: "frame",
      type: {
        kind: "struct",
        fields: [
          {
            name: "timestamp",
            type: "u64",
          },
          {
            name: "bump",
            type: "u8",
          },
        ],
      },
    },
    {
      name: "revenue",
      type: {
        kind: "struct",
        fields: [
          {
            name: "daemon",
            type: "publicKey",
          },
          {
            name: "balance",
            type: "u64",
          },
          {
            name: "bump",
            type: "u8",
          },
        ],
      },
    },
    {
      name: "task",
      type: {
        kind: "struct",
        fields: [
          {
            name: "daemon",
            type: "publicKey",
          },
          {
            name: "id",
            type: "u128",
          },
          {
            name: "instructionData",
            type: {
              defined: "InstructionData",
            },
          },
          {
            name: "status",
            type: {
              defined: "TaskStatus",
            },
          },
          {
            name: "executeAt",
            type: "u64",
          },
          {
            name: "repeatEvery",
            type: "u64",
          },
          {
            name: "repeatUntil",
            type: "u64",
          },
          {
            name: "bump",
            type: "u8",
          },
        ],
      },
    },
    {
      name: "treasury",
      type: {
        kind: "struct",
        fields: [
          {
            name: "bump",
            type: "u8",
          },
        ],
      },
    },
  ],
  types: [
    {
      name: "InstructionData",
      type: {
        kind: "struct",
        fields: [
          {
            name: "programId",
            type: "publicKey",
          },
          {
            name: "keys",
            type: {
              vec: {
                defined: "AccountMetaData",
              },
            },
          },
          {
            name: "data",
            type: "bytes",
          },
        ],
      },
    },
    {
      name: "AccountMetaData",
      type: {
        kind: "struct",
        fields: [
          {
            name: "pubkey",
            type: "publicKey",
          },
          {
            name: "isSigner",
            type: "bool",
          },
          {
            name: "isWritable",
            type: "bool",
          },
        ],
      },
    },
    {
      name: "TaskStatus",
      type: {
        kind: "enum",
        variants: [
          {
            name: "Done",
          },
          {
            name: "Pending",
          },
          {
            name: "Repeat",
          },
        ],
      },
    },
  ],
  errors: [
    {
      code: 6000,
      name: "InvalidSignatory",
      msg: "Your daemon cannot provide all required signatures for this instruction",
    },
    {
      code: 6001,
      name: "TaskNotPending",
      msg: "Task is not pending and may not executed",
    },
    {
      code: 6002,
      name: "TaskNotRepeatable",
      msg: "This task is not marked for repeat",
    },
    {
      code: 6003,
      name: "TaskNotDue",
      msg: "This task is not due and may not be executed yet",
    },
    {
      code: 6004,
      name: "Unknown",
      msg: "Unknown error",
    },
  ],
};
