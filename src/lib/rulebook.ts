import { invoke } from "@tauri-apps/api/core";
import type { Asset, RulebookMeta, RulesetInfo, RulesetInfoWithCovers } from "./types";


export async function getAvailableCoversForRuleset(ruleset: String) {
  return await invoke("get_available_covers_for_ruleset", { ruleset })
}

export async function getRulesetList(): Promise<RulesetInfoWithCovers[]> {
  const rulesets: RulesetInfo[] = await invoke("get_available_rulesets");
  const covers : Asset[][] = await Promise.all(rulesets.map(options => {
    return invoke<Asset[]>("get_available_covers_for_ruleset", {ruleset : options.title})
  }));
  
  const result = rulesets.map((ruleset, index) => ({ ...ruleset, covers: covers[index] } as RulesetInfoWithCovers))
  return result
}