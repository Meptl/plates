/// A wrapper that performs translations before calling the underlying write.
trait FormattedWriter: ::std::io::Write {
    pub fn formatted_write(&mut self, indent) {
        // Replace new lines 
    }
}
