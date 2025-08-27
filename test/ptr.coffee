import "#std.encoding!";

array = rew::ptr::of rew::encoding::stringToBytes "sss"

a = rew::ptr::writeArray array, rew::encoding::stringToBytes "sss"

rew::io::out.print rew::ptr::readArray a, 3