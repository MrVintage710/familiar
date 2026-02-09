
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

export type ItemMeta = {
  name : string,
  desc : string,
  uuid : Uuid,
  type_name : string,
  tags : string[],
}

export type Uuid = `${string}-${string}-${string}-${string}-${string}`;