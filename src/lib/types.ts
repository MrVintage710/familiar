import { v5 as uuidv5 } from 'uuid';

//==============================================================================================
//        Rulebook Types
//==============================================================================================

/**
 * This type mirrors the type provided by the fre. Carries information about a rulebook.
 */
export type RulebookMeta = {
  ruleset: RulesetInfo,
  deps: RulebookDependency[],
  cover? : Asset
}

export type RulesetInfo = {
  title: string,
  short?: string
}

export type RulesetInfoWithCovers = RulesetInfo & { covers : Asset[] }

export type RulebookDependency = {
    rulebook: string,
    version: string,
    uuid : Uuid,
    optional: boolean
}

//==============================================================================================
//        Common Types
//==============================================================================================

export type Asset = {
  data: string,
  mime: string,
  meta: ItemMeta
}

export class ItemMeta {
  name : string
  desc : string = ""
  uuid : Uuid
  type_name : string
  tags: string[] = []
  
  constructor(name : string, typeName : string, tags? : string[], extra? : string) {
    this.name = name;
    this.type_name = typeName;
    this.tags = tags ?? [];
    let namespace = new Uint8Array();
    namespace.set([0x6b, 0xa7, 0xb8, 0x14, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8])
    this.uuid = uuidv5(`${this.name}|${this.type_name}|${extra}`, namespace) as Uuid;
  }
}

export type Uuid = `${string}-${string}-${string}-${string}-${string}`;

//==============================================================================================
//        StatValue
//==============================================================================================

type StatString = { "String": string }

export type StatValue = StatString

//==============================================================================================
//        Constructor
//==============================================================================================

type ConstructorString = { "String": { default: string } }

type ConstructorNumber = { "Number": { default: number } }

type ConstructorInt = { "Int": { default: number } }

type ConstructorChoice = {
  "Choice": {
    values: StatValue,
    number_of_selections: number,
    number_of_choices: number
  }
}

type ConstructorSection = {
  "Section" : {[key : string] : ConstructorValue}
}

export type ConstructorValue =
  ConstructorString | 
  ConstructorNumber |
  ConstructorInt |
  ConstructorChoice |
  ConstructorSection
  
export type ConstructorStep = {
  [key : string] : ConstructorValue
}

export type Constructor = {
  meta: ItemMeta,
  steps: {[key : string] : ConstructorStep}
}

