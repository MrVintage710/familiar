import type { FormEventHandler } from "svelte/elements";

type DebounceEvent = Event & {
  target: EventTarget | null
}

let timerId = $state<number>();

export function debounceOnInput(
  callback: (value: string) => void,
  time : number
) : FormEventHandler<HTMLInputElement> {
  return (event: Event) => {
    clearTimeout(timerId);
    timerId = setTimeout(() => {
      callback((event.target as HTMLInputElement).value)
    }, time)
  }
}