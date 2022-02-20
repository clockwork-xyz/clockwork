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
    pattern: /cronos-sdk = "\d+.\d+.\d+"/,
    formatter: (v: string) => `${CrateNames.sdk} = "${v}"`
  },
  {
    name: CrateNames.program,
    dir: getCargoDir('programs/programs/cronos'),
    pattern: /cronos-program = \{\s?version = \"\d+.\d+.\d+\"/,
    formatter: (v: string) => `${CrateNames.program} = { version = "${v}"`
  }
]

const MEMBERS = ['bot', 'cli', 'programs/programs/cronos', 'sdk']

function getCargoTomlPath(crate: string): string {
  const result = path.resolve(__dirname, '../..', crate, 'Cargo.toml')
return result;
}

function getLatestCrateVersion(crate: typeof Crates[number]) {
  const DEPENDENCY_PATTERN = /version = "\d+.\d+.\d+"/

  const cargoToml = fs.readFileSync(getCargoTomlPath(crate.dir)).toString()
  const versionField = cargoToml.match(DEPENDENCY_PATTERN)?.toString()
  const version = versionField?.replace('version = ', '').replace("\"", '').replace("\"", '')

  invariant(version)


  return version
}

async function read(file:string) {
  const content = await fsp.readFile(file)

  return content.toString()
}

async function bumpCrates() {
  await Promise.all(Crates.map((crate) => {
    const version = getLatestCrateVersion(crate)
    console.log(`[${crate.dir}] latest version: `, version)   

    MEMBERS.forEach(async (dependent) => {
      const dependentCargoTomlPath = getCargoTomlPath(dependent)
      // Read old toml config
      const oldContent = await read(dependentCargoTomlPath)
      // Overrite toml config
      const pattern = new RegExp(`${crate.name} = "\\d+.\\d+.\\d+"`)
      const match = oldContent.match(pattern)?.toString()
      if (!match) return
      const newContent = oldContent.replace(pattern, crate.formatter(version))
      console.log("DEPENDENT: ", dependent)
      console.log('MATCH: ',match)
      await fsp.writeFile(dependentCargoTomlPath, newContent)
    })
  }))

  await execSh.promise('cargo update --workspace')
}

bumpCrates()