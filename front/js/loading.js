let loading_level = 0;
function loadingGetElement() {
    return document.getElementById('preloader');
}
export function loadingStart() {
    if (loading_level == 0) {
        loadingGetElement().classList.remove('scale-out');
        loadingGetElement().classList.add('scale-in');
    }
    loading_level += 1;
}
export function loadingEnd() {
    if (loading_level > 0) {
        loading_level -= 1;
    }
    if (loading_level == 0) {
        loadingGetElement().classList.remove('scale-in');
        loadingGetElement().classList.add('scale-out');
    }
}