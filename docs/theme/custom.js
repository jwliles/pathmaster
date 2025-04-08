// This script ensures the sidebar is expanded and all navigation links work properly

window.addEventListener('load', function() {
    // Expand sidebar items
    document.querySelectorAll('.chapter-item').forEach(function(item) {
        item.classList.add('expanded');
    });

    // Make sure navigation links are working with the site-url prefix
    document.querySelectorAll('a[href^="./"]').forEach(function(link) {
        if (link.href.indexOf('#') === -1 && !link.href.endsWith('.html')) {
            link.href = link.href + '.html';
        }
    });
    
    // Highlight active section
    setTimeout(function() {
        const path = window.location.pathname;
        document.querySelectorAll('.sidebar-scrollbox a').forEach(function(link) {
            if (link.href.includes(path)) {
                link.classList.add('active');
                link.parentElement.classList.add('active');
            }
        });
    }, 100);
});