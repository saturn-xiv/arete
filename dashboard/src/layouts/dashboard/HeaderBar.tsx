import React, { useState } from "react";
import { useDispatch } from "react-redux";
import { useIntl } from "react-intl";
import { useHistory } from "react-router";
import {
  CommandBar,
  PrimaryButton,
  DefaultButton,
  ICommandBarItemProps,
  Dialog,
  DialogType,
  DialogFooter,
  MessageBar,
  MessageBarType,
} from "office-ui-fabric-react";

import { signOut } from "../../actions";
import { delete_ as httpDelete, post as httpPost } from "../../utils/request";

enum Action {
  REBOOT = "reboot",
  SIGN_OUT = "sign-out",
}

interface IDialog {
  body?: string;
  action?: Action;
}

interface IMessageBar {
  type: MessageBarType;
  message: string;
}

const Component = () => {
  const intl = useIntl();
  const history = useHistory();
  const dispatch = useDispatch();

  const [dialog, setDialog] = useState<IDialog>();
  const [messageBar, setMessageBar] = useState<IMessageBar>();

  const showDialog = (act: Action) => {
    setDialog({
      action: act,
      body: intl.formatMessage({
        id: `header.${act}.confirm`,
      }),
    });
  };

  const onDialogSubmit = () => {
    switch (dialog?.action) {
      case Action.REBOOT:
        httpPost("/reboot", {})
          .then(() => {
            setMessageBar({
              type: MessageBarType.success,
              message: intl.formatMessage({ id: "flashes.success" }),
            });
          })
          .catch((e) =>
            setMessageBar({ type: MessageBarType.error, message: e.message })
          );
        break;
      case Action.SIGN_OUT:
        httpDelete("/users/sign-out")
          .then(() => {
            history.push("/users/sign-in");

            dispatch(signOut());
          })
          .catch((e) =>
            setMessageBar({ type: MessageBarType.error, message: e.message })
          );
        break;
    }
    setDialog(undefined);
  };

  const items: ICommandBarItemProps[] = [];

  items.push({
    key: "reboot",
    text: intl.formatMessage({ id: "header.tooltip.reboot" }),
    iconOnly: true,
    iconProps: { iconName: "SyncToPC" },
    onClick: () => {
      showDialog(Action.REBOOT);
    },
  });
  items.push({
    key: "sign-out",
    text: intl.formatMessage({ id: "header.tooltip.sign-out" }),
    iconOnly: true,
    iconProps: { iconName: "SignOut" },
    onClick: () => {
      showDialog(Action.SIGN_OUT);
    },
  });
  return (
    <>
      <CommandBar items={[]} farItems={items} />
      <Dialog
        hidden={dialog?.action === undefined}
        onDismiss={() => setDialog(undefined)}
        dialogContentProps={{
          type: DialogType.normal,
          title: intl.formatMessage({ id: "helper.are-you-sure" }),
          subText: dialog?.body,
        }}
      >
        <DialogFooter>
          <PrimaryButton
            onClick={onDialogSubmit}
            text={intl.formatMessage({ id: "buttons.yes" })}
          />
          <DefaultButton
            onClick={() => setDialog(undefined)}
            text={intl.formatMessage({ id: "buttons.no" })}
          />
        </DialogFooter>
      </Dialog>
      {messageBar && (
        <MessageBar
          messageBarType={messageBar.type}
          isMultiline={false}
          onDismiss={() => setMessageBar(undefined)}
          dismissButtonAriaLabel="Close"
        >
          {messageBar.message}
        </MessageBar>
      )}
    </>
  );
};

export default Component;
