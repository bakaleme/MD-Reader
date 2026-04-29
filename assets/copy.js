(function() {
  'use strict';

  var COPY_SVG = '<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>';
  var CHECK_SVG = '<svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>';

  function getCodeText(btn) {
    var wrapper = btn.closest('.code-block-wrapper');
    if (!wrapper) return '';
    var pre = wrapper.querySelector('pre');
    if (!pre) return '';
    var code = pre.querySelector('code');
    // textContent preserves newlines reliably; innerText can collapse them.
    return (code ? code.textContent : pre.textContent) || '';
  }

  window.copyCode = function(btn) {
    var text = getCodeText(btn);
    if (!text) return;

    var label = btn.querySelector('.copy-label');
    var icon = btn.querySelector('.copy-icon');
    var originalLabel = label ? label.textContent : 'Copy';

    navigator.clipboard.writeText(text).then(function() {
      if (icon) icon.innerHTML = CHECK_SVG;
      if (label) label.textContent = 'Copied!';
      btn.classList.add('copy-success');

      setTimeout(function() {
        if (icon) icon.innerHTML = COPY_SVG;
        if (label) label.textContent = originalLabel;
        btn.classList.remove('copy-success');
      }, 1500);
    }).catch(function(err) {
      // eslint-disable-next-line no-console
      console.error('Clipboard copy failed:', err);
      if (label) label.textContent = 'Failed';
      setTimeout(function() {
        if (label) label.textContent = originalLabel;
      }, 1500);
    });
  };
})();
