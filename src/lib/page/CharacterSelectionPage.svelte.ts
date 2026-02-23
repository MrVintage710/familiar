import type { RulesetInfoWithCovers, Uuid } from "$lib/types";
import { Users, type IconProps } from "@lucide/svelte";
import type { Component } from "svelte";
import { getRulesetList } from "$lib/rulebook";
import { Page } from "./Page.svelte";

export const CHARACTER_PAGE_UUID : Uuid = "424bcb0e-a5ea-401a-9e8f-af581dad365e"

export type CharacterSelectionState = {
  searchTerm : string,
  rulesets: RulesetInfoWithCovers[] | null,
}

export default class CharacterSelectionPage extends Page<CharacterSelectionState> {
  title: string = "Characters";
  icon: Component<IconProps> = Users;
  id: Uuid = CHARACTER_PAGE_UUID;
  
  state: CharacterSelectionState = $state({
    searchTerm: "",
    rulesets: null
  });

  getUrl(): string {
    return "/character"
  }
  
  async onAdd(): Promise<void> {
    getRulesetList().then(rulesets => this.state.rulesets = rulesets)
  }

  async onOpen(): Promise<void> {
    // getRulesetList().then(rulesets => this.state.rulesets = rulesets)
  }
}