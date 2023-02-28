import{o as r,c as o,y as a,q as s,l as p}from"./vendor-43283a12.js";const g="/assets/logo-7f46a3d5.svg";var v={name:"Divider",props:{align:{type:String,default:null},layout:{type:String,default:"horizontal"},type:{type:String,default:"solid"}},computed:{containerClass(){return["p-divider p-component","p-divider-"+this.layout,"p-divider-"+this.type,{"p-divider-left":this.layout==="horizontal"&&(!this.align||this.align==="left")},{"p-divider-center":this.layout==="horizontal"&&this.align==="center"},{"p-divider-right":this.layout==="horizontal"&&this.align==="right"},{"p-divider-top":this.layout==="vertical"&&this.align==="top"},{"p-divider-center":this.layout==="vertical"&&(!this.align||this.align==="center")},{"p-divider-bottom":this.layout==="vertical"&&this.align==="bottom"}]}}};const c=["aria-orientation"],h={key:0,class:"p-divider-content"};function f(t,n,d,i,e,l){return r(),o("div",{class:p(l.containerClass),role:"separator","aria-orientation":d.layout},[t.$slots.default?(r(),o("div",h,[a(t.$slots,"default")])):s("",!0)],10,c)}function y(t,n){n===void 0&&(n={});var d=n.insertAt;if(!(!t||typeof document>"u")){var i=document.head||document.getElementsByTagName("head")[0],e=document.createElement("style");e.type="text/css",d==="top"&&i.firstChild?i.insertBefore(e,i.firstChild):i.appendChild(e),e.styleSheet?e.styleSheet.cssText=t:e.appendChild(document.createTextNode(t))}}var u=`
.p-divider-horizontal {
    display: flex;
    width: 100%;
    position: relative;
    align-items: center;
}
.p-divider-horizontal:before {
    position: absolute;
    display: block;
    top: 50%;
    left: 0;
    width: 100%;
    content: '';
}
.p-divider-horizontal.p-divider-left {
    justify-content: flex-start;
}
.p-divider-horizontal.p-divider-right {
    justify-content: flex-end;
}
.p-divider-horizontal.p-divider-center {
    justify-content: center;
}
.p-divider-content {
    z-index: 1;
}
.p-divider-vertical {
    min-height: 100%;
    margin: 0 1rem;
    display: flex;
    position: relative;
    justify-content: center;
}
.p-divider-vertical:before {
    position: absolute;
    display: block;
    top: 0;
    left: 50%;
    height: 100%;
    content: '';
}
.p-divider-vertical.p-divider-top {
    align-items: flex-start;
}
.p-divider-vertical.p-divider-center {
    align-items: center;
}
.p-divider-vertical.p-divider-bottom {
    align-items: flex-end;
}
.p-divider-solid.p-divider-horizontal:before {
    border-top-style: solid;
}
.p-divider-solid.p-divider-vertical:before {
    border-left-style: solid;
}
.p-divider-dashed.p-divider-horizontal:before {
    border-top-style: dashed;
}
.p-divider-dashed.p-divider-vertical:before {
    border-left-style: dashed;
}
.p-divider-dotted.p-divider-horizontal:before {
    border-top-style: dotted;
}
.p-divider-dotted.p-divider-vertical:before {
    border-left-style: dotted;
}
`;y(u);v.render=f;export{g as _,v as s};
