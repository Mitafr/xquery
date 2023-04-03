import { MenuItem, MenuItemCommandEvent } from "primevue/menuitem";
import { RouteLocationRaw } from "vue-router";

export interface IBreadcrumbItem extends MenuItem {
    readonly label: string,
    readonly icon: string,
}
export interface IBreadcrumb {
    readonly breadcrumbs: Array<MenuItem>
}

export class Breadcrumb implements IBreadcrumb {
    readonly breadcrumbs: Array<IBreadcrumbItem>;

    constructor(breadcrumbs: Array<IBreadcrumbItem>) {
        this.breadcrumbs = breadcrumbs;
    }

}

export class BreadcrumbItem implements IBreadcrumbItem {
    [key: string]: any;
    label: string;
    icon: string;
    to?: RouteLocationRaw | undefined;
    command?: ((event: MenuItemCommandEvent) => void) | undefined;
    url?: string | undefined;
    items?: MenuItem[] | undefined;
    disabled?: boolean | ((...args: any) => boolean) | undefined;
    visible?: boolean | ((...args: any) => boolean) | undefined;
    target?: string | undefined;
    separator?: boolean | undefined;
    style?: any;
    class?: any;
    key?: string | undefined;

    constructor(label: string, icon: string) {
        this.label = label;
        this.icon = icon;
    }
}