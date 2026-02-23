import type { ConstructorStep, RulesetInfo, Uuid } from "$lib/types";
import { UserPlus, type IconProps } from "@lucide/svelte";
import { Page } from "./Page.svelte";
import { v4 as uuidv4 } from "uuid";
import type { Component } from "svelte";
import { cacheCharacterConstructorForRuleset, cachedConstructorRelease, cachedConstructorStep } from "$lib/api/constructor";

export type CreateCharacterPageState = {
  constructorId?: Uuid;
  constructorSteps: ConstructorStep[];
  ruleset: string;
}

export default class CreateCharacterPage extends Page<CreateCharacterPageState> {
  title: string = "New Character";
  icon: Component<IconProps> = UserPlus;
  id: Uuid = uuidv4() as Uuid;
  
  state: CreateCharacterPageState = {
    constructorSteps: [],
    ruleset: ""
  };
  
  static fromRuleset(ruleset: RulesetInfo) {
    const page = new CreateCharacterPage();
    page.state.ruleset = ruleset.short ?? ruleset.title;
    return page;
  }
  
  async onAdd(): Promise<void> {
    this.state.constructorId = await cacheCharacterConstructorForRuleset(this.state.ruleset)
    this.state.constructorSteps[0] = await cachedConstructorStep(this.state.constructorId, 0)
    console.log("State: ", $state.snapshot(this.state));
  }
  
  async onClose(): Promise<void> {
    if (this.state.constructorId) await cachedConstructorRelease(this.state.constructorId)
  }
  
  getUrl(): string {
    return encodeURI(`/character/${this.state.ruleset}/new`);
  }
}