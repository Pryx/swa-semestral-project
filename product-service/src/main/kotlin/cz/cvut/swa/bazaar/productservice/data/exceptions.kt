package cz.cvut.swa.bazaar.productservice.data

class NetworkException(
        override val message: String? = "",
        override val cause: Throwable? = null) : Exception(message, cause)