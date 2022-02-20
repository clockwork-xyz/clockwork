import fs from 'fs'
import fsp from 'fs/promises'
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

function getCargoDir(crate: string): string {
  return path.resolve(__dirname, '../..', crate)
}

/**
 * List of "instructions" used to update Cargo.toml for a specific crate
 */
 const Crates = [
  {
    name: CrateNames.sdk,
    dir: getCargoDir('sdk'),
    pattern: /cronos-sdk = \"\d+.\d+.\d+\"/,
    formatter: (v: string) => `${CrateNames.sdk} = "${v}"`
  },
  {
    name: CrateNames.program,
    dir: getCargoDir('programs/programs/cronos'),
    pattern: /cronos-program = \{\s?version = \"\d+.\d+.\d+\"/,
    formatter: (v: string) => `${CrateNames.program} = { version = "${v}"`
  }
]

function getCargoTomlPath(crate: string): string {
  const result = path.resolve(__dirname, '../..', crate, 'Cargo.toml')
return result;
}

function getLatestCrateVersion(crate: typeof Crates[number]) {
  const pattern = /version = "\d+.\d+.\d+"\n/

  const cargoToml = fs.readFileSync(getCargoTomlPath(crate.dir)).toString()
  const versionField = cargoToml.match(pattern)?.toString()
  const version = versionField?.replace('version = ', '').replace("\"", '').replace("\"", '')

  invariant(version)

  console.log(`[${crate.dir}] latest version: `, version)

  return version
}

async function bumpCrateDependents(crate: typeof Crates[number]) {
  const latestVersion = getLatestCrateVersion(crate)

  await Promise.all(Crates.map(async (dependent) => {
    const dependentCargoTomlPath = getCargoTomlPath(dependent.dir)
    // Read old toml config
    const oldContent = (await fsp.readFile(dependentCargoTomlPath)).toString()
    // Overrite toml config
    const newContent = oldContent.replace(crate.pattern, dependent.formatter(latestVersion))
    await fsp.writeFile(dependentCargoTomlPath, newContent)
  }))
}

async function bumpCrates() {
  await Promise.all(Crates.map(bumpCrateDependents))

  await execSh.promise('cargo update --workspace')
}

bumpCrates()