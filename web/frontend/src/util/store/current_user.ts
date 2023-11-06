import OrcaUser from "../../cls/user";
import { Writable, writable } from "svelte/store";

export const current_user: Writable<OrcaUser | null> = writable(null);
