import {
  AccessibilityElement,
  AccessibilityEvent,
  AccessibilityExtensionAbility,
  FocusDirection,
} from "@kit.AccessibilityKit";
import { hilog } from "@kit.PerformanceAnalysisKit";

const LOG_DOMAIN: number = 0x0000;
const LOG_TAG: string = "AccessibilityE2EReader";

/**
 * Minimal screen-reader-style extension used by the QEMU E2E suite.
 *
 * OpenHarmony's public QEMU phone image does not ship the
 * com.ohos.screenreader application. This extension keeps the test inside the
 * public image while still routing focus and click actions through the real
 * AccessibilityManagerService and AccessibilityExtensionContext.
 */
export default class AccessibilityE2ETestExtension extends AccessibilityExtensionAbility {
  private focusedElement: AccessibilityElement | undefined = undefined;

  onConnect(): void {
    hilog.info(LOG_DOMAIN, LOG_TAG, "connected");
  }

  onDisconnect(): void {
    hilog.info(LOG_DOMAIN, LOG_TAG, "disconnected");
    this.focusedElement = undefined;
  }

  onAccessibilityEvent(event: AccessibilityEvent): void {
    const eventType: string = `${event.eventType}`;
    hilog.info(LOG_DOMAIN, LOG_TAG, "event=%{public}s", eventType);
    switch (eventType) {
      case "right":
        this.moveFocus("forward");
        break;
      case "left":
        this.moveFocus("backward");
        break;
      case "down":
        this.activateFocus();
        break;
      case "hoverEnter":
        if (event.target !== undefined) {
          this.focusHoveredTarget(event.target);
        }
        break;
      default:
        break;
    }
  }

  private moveFocus(direction: FocusDirection): void {
    this.moveFocusAsync(direction).catch((error: Error): void => {
      hilog.error(LOG_DOMAIN, LOG_TAG, "moveFocus failed: %{public}s", error.message);
    });
  }

  private async moveFocusAsync(direction: FocusDirection): Promise<void> {
    let current: AccessibilityElement;
    try {
      current = await this.context.getFocusElement(true);
    } catch {
      current = await this.context.getWindowRootElement();
    }
    const next: AccessibilityElement = await current.findElement("focusDirection", direction);
    await next.performAction("accessibilityFocus");
    this.focusedElement = next;
  }

  private focusHoveredTarget(target: AccessibilityElement): void {
    this.focusHoveredTargetAsync(target).catch((error: Error): void => {
      hilog.error(LOG_DOMAIN, LOG_TAG, "focusHoveredTarget failed: %{public}s", error.message);
    });
  }

  private async focusHoveredTargetAsync(target: AccessibilityElement): Promise<void> {
    const clickable: AccessibilityElement | undefined = await this.findClickable(target, 0);
    if (clickable === undefined) {
      throw new Error("hovered accessibility subtree has no clickable element");
    }
    await clickable.performAction("accessibilityFocus");
    this.focusedElement = clickable;
    hilog.info(LOG_DOMAIN, LOG_TAG, "focused hovered clickable element");
  }

  private async findClickable(
    element: AccessibilityElement,
    depth: number,
  ): Promise<AccessibilityElement | undefined> {
    if (depth > 8) {
      return undefined;
    }
    try {
      const clickable: boolean = await element.attributeValue("clickable");
      if (clickable) {
        const actions: string[] = await element.actionNames();
        if (actions.includes("click")) {
          return element;
        }
      }
    } catch {}

    let children: AccessibilityElement[];
    try {
      children = await element.attributeValue("children");
    } catch {
      return undefined;
    }
    for (const child of children) {
      const target: AccessibilityElement | undefined = await this.findClickable(child, depth + 1);
      if (target !== undefined) {
        return target;
      }
    }
    return undefined;
  }

  private activateFocus(): void {
    this.activateFocusAsync().catch((error: Error): void => {
      hilog.error(LOG_DOMAIN, LOG_TAG, "activateFocus failed: %{public}s", error.message);
    });
  }

  private async activateFocusAsync(): Promise<void> {
    let element: AccessibilityElement;
    try {
      element = await this.context.getFocusElement(true);
    } catch {
      if (this.focusedElement === undefined) {
        throw new Error("no accessibility-focused element");
      }
      element = this.focusedElement;
    }
    await element.performAction("click");
  }
}
