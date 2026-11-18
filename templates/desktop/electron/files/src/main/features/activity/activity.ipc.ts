import { activityChannels, activitySchemas } from "../../../shared/ipc/activity";
import { handle } from "../../ipc/handle";
import { cancelActivity, getActivity, retryActivity, startActivity } from "./activity.service";

export function registerActivityHandlers(): void {
  handle(activityChannels.start, activitySchemas.start, () => startActivity());
  handle(activityChannels.cancel, activitySchemas.cancel, () => cancelActivity());
  handle(activityChannels.retry, activitySchemas.retry, () => retryActivity());
  handle(activityChannels.get, activitySchemas.get, () => getActivity());
}
