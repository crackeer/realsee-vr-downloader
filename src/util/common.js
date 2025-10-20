const startWithProtocol = (url) => {
    return url.startsWith("http://") || url.startsWith("https://");
};
export default {
    startWithProtocol,
}