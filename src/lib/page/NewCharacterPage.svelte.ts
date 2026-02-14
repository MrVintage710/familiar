import type { RulesetInfo, Uuid } from "$lib/types";
import { UserPlus, type IconProps } from "@lucide/svelte";
import { Page, StatefulPage } from "./Page.svelte";
import { v4 as uuidv4 } from "uuid";
import type { Component } from "svelte";

export default class NewCharacterPage extends Page {
  title: string = "New Character";
  icon: Component<IconProps> = UserPlus;
  id: Uuid = uuidv4() as Uuid;
  
  ruleset: string;
  
  constructor(ruleset: RulesetInfo) {
    super();
    this.ruleset = ruleset.short ?? ruleset.title
  }
  
  getUrl(): string {
    console.log(encodeURI(`/character/${this.ruleset}/new`));
    return encodeURI(`/character/${this.ruleset}/new`);
  }
}