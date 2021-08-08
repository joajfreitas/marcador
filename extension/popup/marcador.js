browser.tabs.getCurrent().then((tab) => {
	document.getElementById("url").innerHTML = "ola";
    console.log(tab.url);
}, console.error), 
