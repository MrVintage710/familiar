import type { Uuid } from "$lib/types";
import { CircleQuestionMark, type IconProps } from "@lucide/svelte";
import type { Component } from "svelte";
import { v4 as uuidv4 } from 'uuid';

export abstract class Page<T = any> {
  abstract title: string;
  
  abstract getUrl(): string;
  
  abstract state: T;
  
  icon: Component<IconProps> = CircleQuestionMark;
  id: Uuid = uuidv4() as Uuid;
  
  async onOpen?(): Promise<void>;
  
  async onAdd?(): Promise<void>;
  
  async onClose?(): Promise<void>;
  
  onSave(): $state.Snapshot<T> {
    console.log("Saving", $state.snapshot(this.state))
    return $state.snapshot(this.state)
  }
  
  save(): $state.Snapshot<T> {
    return $state.snapshot(this.state)
  }
}