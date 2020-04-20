package cz.cvut.swa.bazaar.productservice.data.model

import java.time.LocalDateTime

data class Product(

        val id: String? = null,
        val title: String,
        val description: String,
        val postedDatetime: LocalDateTime = LocalDateTime.now()

)