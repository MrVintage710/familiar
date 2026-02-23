import type { Constructor, ConstructorStep, Uuid } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";


export async function cacheCharacterConstructorForRuleset(ruleset : string): Promise<Uuid> {
  const result = await invoke("constructor_cache_character_creator", { ruleset }) as Uuid;
  return result
}

export async function cachedConstructorStep(uuid : Uuid, index : number): Promise<ConstructorStep> {
  const result = await invoke("constructor_get_step", { uuid, index }) as ConstructorStep;
  return result
}

export async function cachedConstructorRelease(uuid: Uuid): Promise<void> {
  try {
    await invoke("constructor_release_ref", { uuid })
  } catch(e) {
    throw e
  }
}