import type { RulesetInfoWithCovers, Uuid } from "$lib/types";
import { Users, type IconProps } from "@lucide/svelte";
import type { Component } from "svelte";
import { getRulesetList } from "$lib/rulebook";
import { Page, type PageData } from "./Page.svelte";

export const CHARACTER_PAGE_UUID : Uuid = "424bcb0e-a5ea-401a-9e8f-af581dad365e"

export type CharacterSelectionState = {
  searchTerm : string,
  rulesets: RulesetInfoWithCovers[] | null,
}

export default class CharacterSelectionPage extends Page implements PageData<CharacterSelectionState> {
  title: string = "Characters";
  icon: Component<IconProps> = Users;
  id: Uuid = CHARACTER_PAGE_UUID;
  
  data: CharacterSelectionState = $state({
    searchTerm: "",
    rulesets: null
  });

  getUrl(): string {
    return "/character"
  }
  
  onAdd(): void {
    getRulesetList().then(rulesets => this.data.rulesets = rulesets)
  }

  onOpen(): void {
    getRulesetList().then(rulesets => this.data.rulesets = rulesets)
  }
  
  onSave(): string {
    return JSON.stringify($state.snapshot(this.data));
  }
}