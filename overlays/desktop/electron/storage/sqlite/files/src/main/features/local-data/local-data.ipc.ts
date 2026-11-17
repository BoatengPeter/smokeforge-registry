import { localDataChannels, localDataSchemas } from "../../../shared/ipc/local-data";
import { handle } from "../../ipc/handle";
import { openLocalDatabase } from "../../storage/database";
import { LocalStore } from "../../storage/local-store";

let store: LocalStore | null = null;

function localStore(): LocalStore {
  store ??= new LocalStore(openLocalDatabase());

  return store;
}

export function registerLocalDataHandlers(): void {
  handle(localDataChannels.get, localDataSchemas.get, (input) => localStore().get(input.key));
  handle(localDataChannels.set, localDataSchemas.set, (input) => {
    localStore().set(input.key, input.value);

    return { stored: true };
  });
  handle(localDataChannels.delete, localDataSchemas.delete, (input) => {
    localStore().delete(input.key);

    return { deleted: true };
  });
}
