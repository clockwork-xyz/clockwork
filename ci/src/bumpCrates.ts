import fs from 'fs'
import path from 'path'
import execSh from "exec-sh";
import { invariant } from './helpers';

/**
 * Names of the crates
 */
const CrateNames = {
  sdk: 'cronos-sdk',
  program: 'cronos-program'
} as const


/**
 * List of "instructions" used to update Cargo.toml for a specific crate
 */
 const Crates = [
  {
    name: CrateNames.sdk,
    dirPath: getCrateDirPath('sdk'),
    version: getLatestCrateVersion('sdk'),
    pattern: /cronos-sdk = \"\d+.\d+.\d+\"/,
    formatter: (v: string) => `${CrateNames.sdk} = "${v}"`
  },
  {
    name: CrateNames.program,
    dirPath: getCrateDirPath('programs/programs/cronos'),
    version: getLatestCrateVersion(CrateNames.program),
    pattern: /cronos-program = \{\s?version = \"\d+.\d+.\d+\"/,
    formatter: (v: string) => `${CrateNames.program} = { version = "${v}"`
  }
]


function getCrateDirPath(crate: string): string {
  return path.resolve(__dirname, '../..', crate)
}

function getCargoTomlPath(crate: string): string {
  return path.resolve(__dirname, '../..', crate, 'Cargo.toml')
}

function getLatestCrateVersion(dirPath: string) {
  const pattern = /version = "\d+.\d+.\d+"\n/

  const cargoToml = fs.readFileSync(getCargoTomlPath(dirPath)).toString()
  const versionField = cargoToml.match(pattern)?.toString()
  const version = versionField?.replace('version = ', '').replace("\"", '').replace("\"", '')

  invariant(version)

  console.log(`dirPath[${dirPath}] latest version: `, version)

  return version
}

async function bumpCrates() {
  await Promise.all(Crates.map(({ name, version, pattern, formatter }) => {

    const crateDir = path.resolve(__dirname, '../..', name)
    const filePath = path.resolve(__dirname, '../..', name, 'Cargo.toml')

    return new Promise(async (resolve, reject) => {
      const oldContent = fs.readFileSync(filePath).toString()

      try {
        Crates.forEach(async ({
          dirPath,
          version,
          pattern,
          formatter
        }) => {
          // Overrite toml config
          const newContent = oldContent.replace(pattern, formatter(version))
          fs.writeFileSync(filePath, newContent)

          // Re-generate lockfile
          const lockFilePath = path.resolve(dirPath, 'Cargo.lock')
          const exists = fs.existsSync(lockFilePath)
          exists && await execSh.promise(`cd ${crateDir} && rm -rf Cargo.lock && cargo update && cd ..`)
        })
        resolve(null)
      } catch (err) {
        reject(err)
      }
    })
  }))
}

bumpCrates()