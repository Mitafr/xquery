import{Z as y,D as b,C as U,U as P,r as x,o as r,c as d,f as c,x as w,l as h,q as C,e as a,t as _,w as k,v as H,y as S,n as Z,d as V,$ as q,z as p,a0 as L,Q as E,Y as K,_ as F,j as G,F as R,m as T,s as N,p as D,a1 as g,O as $,a2 as j,k as Y,B as W,u as X,a3 as Q,a as J}from"./vendor-43283a12.js";import{s as ee,_ as te}from"./divider.esm-f91c4a07.js";import{O as se,s as A,a as ne,R as le}from"./portal.esm-ae6bd3f1.js";import{s as B}from"./inputtext.esm-0dd8cff0.js";import{_ as ie,s as ae}from"./index-25d091d5.js";var z={name:"Password",emits:["update:modelValue","change","focus","blur","invalid"],props:{modelValue:String,promptLabel:{type:String,default:null},mediumRegex:{type:String,default:"^(((?=.*[a-z])(?=.*[A-Z]))|((?=.*[a-z])(?=.*[0-9]))|((?=.*[A-Z])(?=.*[0-9])))(?=.{6,})"},strongRegex:{type:String,default:"^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.{8,})"},weakLabel:{type:String,default:null},mediumLabel:{type:String,default:null},strongLabel:{type:String,default:null},feedback:{type:Boolean,default:!0},appendTo:{type:String,default:"body"},toggleMask:{type:Boolean,default:!1},hideIcon:{type:String,default:"pi pi-eye-slash"},showIcon:{type:String,default:"pi pi-eye"},disabled:{type:Boolean,default:!1},placeholder:{type:String,default:null},required:{type:Boolean,default:!1},inputId:{type:String,default:null},inputClass:{type:String,default:null},inputStyle:{type:null,default:null},inputProps:{type:null,default:null},panelId:{type:String,default:null},panelClass:{type:String,default:null},panelStyle:{type:null,default:null},panelProps:{type:null,default:null},"aria-labelledby":{type:String,default:null},"aria-label":{type:String,default:null}},data(){return{overlayVisible:!1,meter:null,infoText:null,focused:!1,unmasked:!1}},mediumCheckRegExp:null,strongCheckRegExp:null,resizeListener:null,scrollHandler:null,overlay:null,mounted(){this.infoText=this.promptText,this.mediumCheckRegExp=new RegExp(this.mediumRegex),this.strongCheckRegExp=new RegExp(this.strongRegex)},beforeUnmount(){this.unbindResizeListener(),this.scrollHandler&&(this.scrollHandler.destroy(),this.scrollHandler=null),this.overlay&&(y.clear(this.overlay),this.overlay=null)},methods:{onOverlayEnter(e){y.set("overlay",e,this.$primevue.config.zIndex.overlay),this.alignOverlay(),this.bindScrollListener(),this.bindResizeListener()},onOverlayLeave(){this.unbindScrollListener(),this.unbindResizeListener(),this.overlay=null},onOverlayAfterLeave(e){y.clear(e)},alignOverlay(){this.appendTo==="self"?b.relativePosition(this.overlay,this.$refs.input.$el):(this.overlay.style.minWidth=b.getOuterWidth(this.$refs.input.$el)+"px",b.absolutePosition(this.overlay,this.$refs.input.$el))},testStrength(e){let s=0;return this.strongCheckRegExp.test(e)?s=3:this.mediumCheckRegExp.test(e)?s=2:e.length&&(s=1),s},onInput(e){this.$emit("update:modelValue",e.target.value)},onFocus(e){this.focused=!0,this.feedback&&(this.setPasswordMeter(this.modelValue),this.overlayVisible=!0),this.$emit("focus",e)},onBlur(e){this.focused=!1,this.feedback&&(this.overlayVisible=!1),this.$emit("blur",e)},onKeyUp(e){if(this.feedback){const s=e.target.value,{meter:t,label:i}=this.checkPasswordStrength(s);if(this.meter=t,this.infoText=i,e.code==="Escape"){this.overlayVisible&&(this.overlayVisible=!1);return}this.overlayVisible||(this.overlayVisible=!0)}},setPasswordMeter(){if(!this.modelValue)return;const{meter:e,label:s}=this.checkPasswordStrength(this.modelValue);this.meter=e,this.infoText=s,this.overlayVisible||(this.overlayVisible=!0)},checkPasswordStrength(e){let s=null,t=null;switch(this.testStrength(e)){case 1:s=this.weakText,t={strength:"weak",width:"33.33%"};break;case 2:s=this.mediumText,t={strength:"medium",width:"66.66%"};break;case 3:s=this.strongText,t={strength:"strong",width:"100%"};break;default:s=this.promptText,t=null;break}return{label:s,meter:t}},onInvalid(e){this.$emit("invalid",e)},bindScrollListener(){this.scrollHandler||(this.scrollHandler=new U(this.$refs.input.$el,()=>{this.overlayVisible&&(this.overlayVisible=!1)})),this.scrollHandler.bindScrollListener()},unbindScrollListener(){this.scrollHandler&&this.scrollHandler.unbindScrollListener()},bindResizeListener(){this.resizeListener||(this.resizeListener=()=>{this.overlayVisible&&!b.isTouchDevice()&&(this.overlayVisible=!1)},window.addEventListener("resize",this.resizeListener))},unbindResizeListener(){this.resizeListener&&(window.removeEventListener("resize",this.resizeListener),this.resizeListener=null)},overlayRef(e){this.overlay=e},onMaskToggle(){this.unmasked=!this.unmasked},onOverlayClick(e){se.emit("overlay-click",{originalEvent:e,target:this.$el})}},computed:{containerClass(){return["p-password p-component p-inputwrapper",{"p-inputwrapper-filled":this.filled,"p-inputwrapper-focus":this.focused,"p-input-icon-right":this.toggleMask}]},inputFieldClass(){return["p-password-input",this.inputClass,{"p-disabled":this.disabled}]},panelStyleClass(){return["p-password-panel p-component",this.panelClass,{"p-input-filled":this.$primevue.config.inputStyle==="filled","p-ripple-disabled":this.$primevue.config.ripple===!1}]},toggleIconClass(){return this.unmasked?this.hideIcon:this.showIcon},strengthClass(){return`p-password-strength ${this.meter?this.meter.strength:""}`},inputType(){return this.unmasked?"text":"password"},filled(){return this.modelValue!=null&&this.modelValue.toString().length>0},weakText(){return this.weakLabel||this.$primevue.config.locale.weak},mediumText(){return this.mediumLabel||this.$primevue.config.locale.medium},strongText(){return this.strongLabel||this.$primevue.config.locale.strong},promptText(){return this.promptLabel||this.$primevue.config.locale.passwordPrompt},panelUniqueId(){return P()+"_panel"}},components:{PInputText:B,Portal:A}};const oe={class:"p-hidden-accessible","aria-live":"polite"},re=["id"],ue={class:"p-password-meter"},de={class:"p-password-info"};function ce(e,s,t,i,l,n){const o=x("PInputText"),u=x("Portal");return r(),d("div",{class:h(n.containerClass)},[c(o,w({ref:"input",id:t.inputId,type:n.inputType,class:n.inputFieldClass,style:t.inputStyle,value:t.modelValue,"aria-labelledby":e.ariaLabelledby,"aria-label":e.ariaLabel,"aria-controls":t.panelProps&&t.panelProps.id||t.panelId||n.panelUniqueId,"aria-expanded":l.overlayVisible,"aria-haspopup":!0,placeholder:t.placeholder,required:t.required,onInput:n.onInput,onFocus:n.onFocus,onBlur:n.onBlur,onKeyup:n.onKeyUp,onInvalid:n.onInvalid},t.inputProps),null,16,["id","type","class","style","value","aria-labelledby","aria-label","aria-controls","aria-expanded","placeholder","required","onInput","onFocus","onBlur","onKeyup","onInvalid"]),t.toggleMask?(r(),d("i",{key:0,class:h(n.toggleIconClass),onClick:s[0]||(s[0]=(...m)=>n.onMaskToggle&&n.onMaskToggle(...m))},null,2)):C("",!0),a("span",oe,_(l.infoText),1),c(u,{appendTo:t.appendTo},{default:k(()=>[c(H,{name:"p-connected-overlay",onEnter:n.onOverlayEnter,onLeave:n.onOverlayLeave,onAfterLeave:n.onOverlayAfterLeave},{default:k(()=>[l.overlayVisible?(r(),d("div",w({key:0,ref:n.overlayRef,id:t.panelId||n.panelUniqueId,class:n.panelStyleClass,style:t.panelStyle,onClick:s[1]||(s[1]=(...m)=>n.onOverlayClick&&n.onOverlayClick(...m))},t.panelProps),[S(e.$slots,"header"),S(e.$slots,"content",{},()=>[a("div",ue,[a("div",{class:h(n.strengthClass),style:Z({width:l.meter?l.meter.width:""})},null,6)]),a("div",de,_(l.infoText),1)]),S(e.$slots,"footer")],16,re)):C("",!0)]),_:3},8,["onEnter","onLeave","onAfterLeave"])]),_:3},8,["appendTo"])],2)}function pe(e,s){s===void 0&&(s={});var t=s.insertAt;if(!(!e||typeof document>"u")){var i=document.head||document.getElementsByTagName("head")[0],l=document.createElement("style");l.type="text/css",t==="top"&&i.firstChild?i.insertBefore(l,i.firstChild):i.appendChild(l),l.styleSheet?l.styleSheet.cssText=e:l.appendChild(document.createTextNode(e))}}var me=`
.p-password {
    position: relative;
    display: inline-flex;
}
.p-password-panel {
    position: absolute;
    top: 0;
    left: 0;
}
.p-password .p-password-panel {
    min-width: 100%;
}
.p-password-meter {
    height: 10px;
}
.p-password-strength {
    height: 100%;
    width: 0;
    transition: width 1s ease-in-out;
}
.p-fluid .p-password {
    display: flex;
}
.p-password-input::-ms-reveal,
.p-password-input::-ms-clear {
    display: none;
}
`;pe(me);z.render=ce;const I=e=>(K("data-v-7ad61875"),e=e(),F(),e),he={class:"col-12 md:col-8 lg:col-6 xl:col-4 p-fluid m-auto"},fe={class:"surface-card p-4 shadow-8 border-round w-full"},ge={class:"text-center mb-5"},ye=I(()=>a("img",{src:te,alt:"Image",height:"80",class:"mb-3"},null,-1)),ve=I(()=>a("div",{class:"text-900 text-3xl font-medium mb-3"},"Connexion",-1)),be=I(()=>a("label",{for:"username",class:"block text-900 font-medium mb-2"},"Username",-1)),xe=I(()=>a("label",{for:"password",class:"block text-900 font-medium mb-2"},"Password",-1)),we={class:"formgrid grid"},_e={class:"field col m-auto md:col-8 sm:col-12"},ke=V({__name:"AuthLogin",props:{loading:{type:Boolean}},emits:["login"],setup(e,{emit:s}){const t=e;let i="user01",l="password1";const n=q(t,"loading"),o=async function(){u()&&s("login",{username:i,password:l})},u=function(){return i.length>0&&l.length>0};return(m,f)=>(r(),d("div",he,[a("div",fe,[a("div",ge,[ye,ve,c(p(ee))]),a("div",null,[be,c(p(B),{id:"username",type:"text",modelValue:p(i),"onUpdate:modelValue":f[0]||(f[0]=v=>L(i)?i.value=v:i=v),class:"w-full mb-3",onKeyup:E(o,["enter"])},null,8,["modelValue","onKeyup"]),xe,c(p(z),{id:"password",modelValue:p(l),"onUpdate:modelValue":f[1]||(f[1]=v=>L(l)?l.value=v:l=v),toggleMask:"",class:"w-full mb-3",onKeyup:E(o,["enter"]),feedback:!1},null,8,["modelValue","onKeyup"]),a("div",we,[a("div",_e,[c(p(ne),{label:"Submit",icon:"pi pi-send",iconPos:"right",class:"p-button p-button-rounded p-button-success",onClick:o,loading:p(n)},null,8,["loading"])])])])])]))}}),Ie=ie(ke,[["__scopeId","data-v-7ad61875"]]);var O={name:"ToastMessage",emits:["close"],props:{message:{type:null,default:null},template:{type:null,default:null},closeIcon:{type:String,default:null},infoIcon:{type:String,default:null},warnIcon:{type:String,default:null},errorIcon:{type:String,default:null},successIcon:{type:String,default:null},closeButtonProps:{type:null,default:null}},closeTimeout:null,mounted(){this.message.life&&(this.closeTimeout=setTimeout(()=>{this.close()},this.message.life))},beforeUnmount(){this.clearCloseTimeout()},methods:{close(){this.$emit("close",this.message)},onCloseClick(){this.clearCloseTimeout(),this.close()},clearCloseTimeout(){this.closeTimeout&&(clearTimeout(this.closeTimeout),this.closeTimeout=null)}},computed:{containerClass(){return["p-toast-message",this.message.styleClass,{"p-toast-message-info":this.message.severity==="info","p-toast-message-warn":this.message.severity==="warn","p-toast-message-error":this.message.severity==="error","p-toast-message-success":this.message.severity==="success"}]},iconClass(){return["p-toast-message-icon",{[this.infoIcon]:this.message.severity==="info",[this.warnIcon]:this.message.severity==="warn",[this.errorIcon]:this.message.severity==="error",[this.successIcon]:this.message.severity==="success"}]},closeAriaLabel(){return this.$primevue.config.locale.aria?this.$primevue.config.locale.aria.close:void 0}},directives:{ripple:le}};const Se={class:"p-toast-message-text"},Ce={class:"p-toast-summary"},Te={class:"p-toast-detail"},Le={key:2},Ee=["aria-label"];function Pe(e,s,t,i,l,n){const o=G("ripple");return r(),d("div",{class:h(n.containerClass),role:"alert","aria-live":"assertive","aria-atomic":"true"},[a("div",{class:h(["p-toast-message-content",t.message.contentStyleClass])},[t.template?(r(),T(N(t.template),{key:1,message:t.message},null,8,["message"])):(r(),d(R,{key:0},[a("span",{class:h(n.iconClass)},null,2),a("div",Se,[a("span",Ce,_(t.message.summary),1),a("div",Te,_(t.message.detail),1)])],64)),t.message.closable!==!1?(r(),d("div",Le,[D((r(),d("button",w({class:"p-toast-icon-close p-link",type:"button","aria-label":n.closeAriaLabel,onClick:s[0]||(s[0]=(...u)=>n.onCloseClick&&n.onCloseClick(...u)),autofocus:""},t.closeButtonProps),[a("span",{class:h(["p-toast-icon-close-icon",t.closeIcon])},null,2)],16,Ee)),[[o]])])):C("",!0)],2)],2)}O.render=Pe;var Ve=0,M={name:"Toast",inheritAttrs:!1,props:{group:{type:String,default:null},position:{type:String,default:"top-right"},autoZIndex:{type:Boolean,default:!0},baseZIndex:{type:Number,default:0},breakpoints:{type:Object,default:null},closeIcon:{type:String,default:"pi pi-times"},infoIcon:{type:String,default:"pi pi-info-circle"},warnIcon:{type:String,default:"pi pi-exclamation-triangle"},errorIcon:{type:String,default:"pi pi-times"},successIcon:{type:String,default:"pi pi-check"},closeButtonProps:{type:null,default:null}},data(){return{messages:[]}},styleElement:null,mounted(){g.on("add",this.onAdd),g.on("remove-group",this.onRemoveGroup),g.on("remove-all-groups",this.onRemoveAllGroups),this.breakpoints&&this.createStyle()},beforeUnmount(){this.destroyStyle(),this.$refs.container&&this.autoZIndex&&y.clear(this.$refs.container),g.off("add",this.onAdd),g.off("remove-group",this.onRemoveGroup),g.off("remove-all-groups",this.onRemoveAllGroups)},methods:{add(e){e.id==null&&(e.id=Ve++),this.messages=[...this.messages,e]},remove(e){let s=-1;for(let t=0;t<this.messages.length;t++)if(this.messages[t]===e){s=t;break}this.messages.splice(s,1)},onAdd(e){this.group==e.group&&this.add(e)},onRemoveGroup(e){this.group===e&&(this.messages=[])},onRemoveAllGroups(){this.messages=[]},onEnter(){this.$refs.container.setAttribute(this.attributeSelector,""),this.autoZIndex&&y.set("modal",this.$refs.container,this.baseZIndex||this.$primevue.config.zIndex.modal)},onLeave(){this.$refs.container&&this.autoZIndex&&$.isEmpty(this.messages)&&setTimeout(()=>{y.clear(this.$refs.container)},200)},createStyle(){if(!this.styleElement){this.styleElement=document.createElement("style"),this.styleElement.type="text/css",document.head.appendChild(this.styleElement);let e="";for(let s in this.breakpoints){let t="";for(let i in this.breakpoints[s])t+=i+":"+this.breakpoints[s][i]+"!important;";e+=`
                        @media screen and (max-width: ${s}) {
                            .p-toast[${this.attributeSelector}] {
                                ${t}
                            }
                        }
                    `}this.styleElement.innerHTML=e}},destroyStyle(){this.styleElement&&(document.head.removeChild(this.styleElement),this.styleElement=null)}},computed:{containerClass(){return["p-toast p-component p-toast-"+this.position,{"p-input-filled":this.$primevue.config.inputStyle==="filled","p-ripple-disabled":this.$primevue.config.ripple===!1}]},attributeSelector(){return P()}},components:{ToastMessage:O,Portal:A}};function Re(e,s,t,i,l,n){const o=x("ToastMessage"),u=x("Portal");return r(),T(u,null,{default:k(()=>[a("div",w({ref:"container",class:n.containerClass},e.$attrs),[c(j,{name:"p-toast-message",tag:"div",onEnter:n.onEnter,onLeave:n.onLeave},{default:k(()=>[(r(!0),d(R,null,Y(l.messages,m=>(r(),T(o,{key:m.id,message:m,template:e.$slots.message,closeIcon:t.closeIcon,infoIcon:t.infoIcon,warnIcon:t.warnIcon,errorIcon:t.errorIcon,successIcon:t.successIcon,closeButtonProps:t.closeButtonProps,onClose:s[0]||(s[0]=f=>n.remove(f))},null,8,["message","template","closeIcon","infoIcon","warnIcon","errorIcon","successIcon","closeButtonProps"]))),128))]),_:1},8,["onEnter","onLeave"])],16)]),_:1})}function Ae(e,s){s===void 0&&(s={});var t=s.insertAt;if(!(!e||typeof document>"u")){var i=document.head||document.getElementsByTagName("head")[0],l=document.createElement("style");l.type="text/css",t==="top"&&i.firstChild?i.insertBefore(l,i.firstChild):i.appendChild(l),l.styleSheet?l.styleSheet.cssText=e:l.appendChild(document.createTextNode(e))}}var Be=`
.p-toast {
    position: fixed;
    width: 25rem;
}
.p-toast-message-content {
    display: flex;
    align-items: flex-start;
}
.p-toast-message-text {
    flex: 1 1 auto;
}
.p-toast-top-right {
    top: 20px;
    right: 20px;
}
.p-toast-top-left {
    top: 20px;
    left: 20px;
}
.p-toast-bottom-left {
    bottom: 20px;
    left: 20px;
}
.p-toast-bottom-right {
    bottom: 20px;
    right: 20px;
}
.p-toast-top-center {
    top: 20px;
    left: 50%;
    transform: translateX(-50%);
}
.p-toast-bottom-center {
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
}
.p-toast-center {
    left: 50%;
    top: 50%;
    min-width: 20vw;
    transform: translate(-50%, -50%);
}
.p-toast-icon-close {
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    position: relative;
}
.p-toast-icon-close.p-link {
    cursor: pointer;
}

/* Animations */
.p-toast-message-enter-from {
    opacity: 0;
    -webkit-transform: translateY(50%);
    -ms-transform: translateY(50%);
    transform: translateY(50%);
}
.p-toast-message-leave-from {
    max-height: 1000px;
}
.p-toast .p-toast-message.p-toast-message-leave-to {
    max-height: 0;
    opacity: 0;
    margin-bottom: 0;
    overflow: hidden;
}
.p-toast-message-enter-active {
    -webkit-transition: transform 0.3s, opacity 0.3s;
    transition: transform 0.3s, opacity 0.3s;
}
.p-toast-message-leave-active {
    -webkit-transition: max-height 0.45s cubic-bezier(0, 1, 0, 1), opacity 0.3s, margin-bottom 0.3s;
    transition: max-height 0.45s cubic-bezier(0, 1, 0, 1), opacity 0.3s, margin-bottom 0.3s;
}
`;Ae(Be);M.render=Re;const ze={class:"grid h-screen w-screen m-0"},qe=V({__name:"Login",setup(e){let s=W(!1);const t=ae,i=X(),l=Q(),n=async o=>{try{s.value=!0,await t.postLogin(o.username,o.password)}catch(u){J.isAxiosError(u)&&u.response&&(l.removeAllGroups(),l.add({severity:"error",summary:u.response.data,life:6e3}))}finally{s.value=!1,i.push("/")}};return t.user.authenticated&&i.push("/"),(o,u)=>(r(),d("div",ze,[c(Ie,{onLogin:n,loading:p(s)},null,8,["loading"]),c(p(M),{position:"top-center"})]))}});export{qe as default};
